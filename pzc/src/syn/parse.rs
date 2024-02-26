//! The parser.

use std::mem;

use crate::syn;
use crate::syn::ImportedSymbol;

use ilex::rule;
use ilex::token;
use ilex::token::Cursor;
use ilex::Lexeme;
use ilex::Report;
use ilex::Span;
use ilex::Spanned;
use ilex::Token;

#[ilex::spec]
struct Pz {
  #[named("line comment")]
  #[rule("//")]
  line_comment: Lexeme<rule::Comment>,

  #[named("doc comment")]
  #[rule(rule::Quoted::from(rule::Bracket::paired("///", "\n")))]
  doc_comment: Lexeme<rule::Quoted>,

  #[named("block comment")]
  #[rule("/*", "*/")]
  block_comment: Lexeme<rule::Comment>,

  #[rule("{", "}")]
  braces: Lexeme<rule::Bracket>,

  #[rule(";")]
  semi: Lexeme<rule::Keyword>,
  #[rule(".")]
  dot: Lexeme<rule::Keyword>,
  #[rule("=")]
  equal: Lexeme<rule::Keyword>,
  #[rule(":")]
  colon: Lexeme<rule::Keyword>,
  #[rule("/")]
  slash: Lexeme<rule::Keyword>,
  #[rule(",")]
  comma: Lexeme<rule::Keyword>,
  #[rule("@")]
  at: Lexeme<rule::Keyword>,

  #[rule("enum")]
  enum_: Lexeme<rule::Keyword>,
  #[rule("struct")]
  struct_: Lexeme<rule::Keyword>,
  #[rule("where")]
  where_: Lexeme<rule::Keyword>,
  #[rule("as")]
  as_: Lexeme<rule::Keyword>,

  message: Lexeme<rule::Keyword>,
  choice: Lexeme<rule::Keyword>,
  package: Lexeme<rule::Keyword>,
  repeated: Lexeme<rule::Keyword>,
  import: Lexeme<rule::Keyword>,

  #[named("identifier")]
  #[rule(rule::Ident::new().ascii_only().prefixes(["", "#"]))]
  ident: Lexeme<rule::Ident>,

  #[named("integer")]
  #[rule(rule::Digital::new(10).separator("_").minus())]
  int: Lexeme<rule::Digital>,

  #[named("binary integer")]
  #[rule(rule::Digital::new(2).separator("_").minus().prefix("0b"))]
  bin: Lexeme<rule::Digital>,

  #[named("hexadecimal integer")]
  #[rule(rule::Digital::new(16).separator("_").minus().prefix("0x"))]
  hex: Lexeme<rule::Digital>,

  #[named("string literal")]
  #[rule(rule::Quoted::new('"').invalid_escape(r"\").escapes(["\\\"", r"\\"]))]
  str: Lexeme<rule::Quoted>,
}

fn parse_path<'t>(
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
) -> Option<syn::Path<'t>> {
  let mut path = syn::Path {
    components: vec![(toks.take(pz.ident, report)?, None)],
  };

  while let Some(dot) = toks.try_take(pz.dot) {
    path.components.last_mut().unwrap().1 = Some(dot);
    path.components.push((toks.take(pz.ident, report)?, None));
  }

  Some(path)
}

fn parse_ident<'t>(
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
) -> Option<token::Ident<'t>> {
  // We go through parse_path to catch a path where we wanted a single
  // identifier.
  let idents = parse_path(pz, toks, report)?;

  if idents.components.len() > 1 {
    report.error("expected identifier, got path").at(&idents);
  }

  Some(idents.components[0].0)
}

fn parse_package<'t>(
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
) -> Option<(Vec<syn::Attr<'t>>, syn::Package<'t>)> {
  let mut attrs = Vec::new();
  loop {
    let result = token::switch()
      .case(pz.doc_comment, |doc, _| Err(Some(syn::Attr::Doc(doc))))
      .case(pz.at, |at, toks| {
        Err(parse_attr_kv(Some(at), pz, toks, report))
      })
      .case(pz.package, |package, toks| {
        Ok(
          parse_path(pz, toks, report)
            .map(|path| syn::Package { package, path }),
        )
      })
      .take(toks, report)?;

    match result {
      Ok(pkg) => return Some((attrs, pkg?)),
      Err(attr) => attrs.push(attr?),
    }
  }
}

fn parse_type<'t>(
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
) -> Option<syn::Type<'t>> {
  if let Some(repeated) = toks.try_take(pz.repeated) {
    return Some(syn::Type::Repeated {
      repeated,
      element: Box::new(parse_type(pz, toks, report)?),
    });
  };

  Some(syn::Type::Path(parse_path(pz, toks, report)?))
}

fn parse_attr_kv<'t>(
  at: Option<token::Keyword<'t>>,
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
) -> Option<syn::Attr<'t>> {
  let name = parse_path(pz, toks, report);
  let value = toks.try_take(pz.equal).and_then(|eq| {
    let value = token::switch()
      .case(pz.ident, |_, toks| {
        toks.back_up(1);
        parse_path(pz, toks, report).map(syn::AttrValue::Path)
      })
      .case(pz.str, |q, _| Some(syn::AttrValue::Str(q)))
      .cases([pz.int, pz.bin, pz.hex], |d, _| {
        Some(syn::AttrValue::Int(d))
      })
      .take(toks, report)
      .flatten();
    value.map(|v| (eq, v))
  });

  Some(syn::Attr::At {
    at,
    name: name?,
    value,
  })
}

fn parse_wheres<'t>(
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
  attrs: &mut Vec<syn::Attr<'t>>,
) {
  while let Some(_where_) = toks.try_take(pz.where_) {
    if let Some(attr) = parse_attr_kv(None, pz, toks, report) {
      attrs.push(attr);
    }
  }
}

fn parse_item<'t>(
  pz: &Pz,
  toks: &mut Cursor<'t>,
  report: &Report,
  mut imports: Option<&mut Vec<syn::Import<'t>>>,
  inside: Option<syn::DeclKind>,
  outer_name: Option<token::Ident<'t>>,
) -> Option<syn::Item<'t>> {
  let mut attrs = Vec::new();
  let inside_of = match inside {
    Some(syn::DeclKind::Message) => "message",
    Some(syn::DeclKind::Enum) => "enum",
    Some(syn::DeclKind::Struct) => "struct",
    Some(syn::DeclKind::Choice) => "choice",
    None => "file",
  };

  loop {
    enum What<'t> {
      Attr(syn::Attr<'t>),
      Import(syn::Import<'t>),
      Decl(syn::Decl<'t>),
      Field(syn::Field<'t>),
    }

    let done = token::switch()
      .case(pz.at, |at, toks| {
        Some(What::Attr(parse_attr_kv(Some(at), pz, toks, report)?))
      })
      .case(pz.doc_comment, |doc, _| {
        Some(What::Attr(syn::Attr::Doc(doc)))
      })
      .case(pz.import, |import, toks| {
        let package = parse_path(pz, toks, report);
        let braces = toks.take(pz.braces, report);

        let mut symbols = Vec::new();
        if let Some(braces) = braces {
          symbols.extend(
            braces
              .contents()
              .delimited(pz.comma, |toks| {
                let symbol = parse_path(pz, toks, report);
                let rename = toks
                  .try_take(pz.as_)
                  .and_then(|as_| Some((as_, parse_ident(pz, toks, report)?)));
                Some(ImportedSymbol {
                  symbol: symbol?,
                  rename,
                })
              })
              .map(|(i, _)| i),
          );
        }

        Some(What::Import(syn::Import {
          import,
          attrs: Vec::new(),
          package: package?,
          braces: braces?,
          symbols,
        }))
      })
      .cases([pz.message, pz.enum_, pz.struct_, pz.choice], |kw, toks| {
        let kind = if kw.lexeme() == pz.message {
          syn::DeclKind::Message
        } else if kw.lexeme() == pz.enum_ {
          syn::DeclKind::Enum
        } else if kw.lexeme() == pz.struct_ {
          syn::DeclKind::Struct
        } else {
          syn::DeclKind::Choice
        };

        let name = parse_ident(pz, toks, report);
        let body = toks.take(pz.braces, report);

        let outer_span = body.map(|body| {
          Span::union([kw.span(toks.context()), body.span(toks.context())])
        });

        if inside == Some(syn::DeclKind::Enum) {
          let mut d =
            report.error("declarations not permitted inside an `enum`");
          if let Some(span) = outer_span {
            d = d.saying(span, "this declaration");
          }
          if let Some(outer) = outer_name {
            d.remark(outer, "declared within this enum");
          }
        }

        let mut items = Vec::new();
        if let Some(body) = body {
          let mut toks = body.contents();
          while !toks.is_empty() {
            if let Some(item) =
              parse_item(pz, &mut toks, report, None, Some(kind), name)
            {
              items.push(item);
            }
          }
        }

        Some(What::Decl(syn::Decl {
          kw,
          braces: body?,
          kind,
          name: name?,
          items,
          attrs: Vec::new(),
        }))
      })
      .cases(
        [pz.ident.any(), pz.int.any(), pz.hex.any(), pz.bin.any()],
        |tok, toks| {
          let number = tok.digital().ok();
          if let Some(n) = number {
            if inside == Some(syn::DeclKind::Struct) {
              let d = report
                .error(format_args!("struct fields cannot have numbers"))
                .saying(n, "remove this number");

              if let Some(outer) = outer_name {
                d.remark(outer, format_args!("declared within this struct"));
              }
            }

            toks.take(pz.dot, report);
          } else {
            toks.back_up(1);
          };

          let name = parse_ident(pz, toks, report);

          // This needs to be here so we can moor the diagnostic onto the `name`.
          // (This may be a sign I should move where these diagnostics are
          // generated...)
          if number.is_none() && inside != Some(syn::DeclKind::Struct) {
            let mut d = report
              .error(format_args!("{inside_of} fields must have numbers"));
            if let Some(name) = name {
              d = d.saying(name, "expected number")
            }
            if let Some(outer) = outer_name {
              d.remark(outer, format_args!("declared within this {inside_of}"));
            }
          }

          let mut ty = None;
          if let Some(_colon) = toks.try_take(pz.colon) {
            ty = parse_type(pz, toks, report);
            if inside == Some(syn::DeclKind::Enum) {
              let mut d = report
                .error(format_args!("{inside_of} fields cannot have types"));
              if let Some(ty) = &ty {
                d = d.saying(ty, "remove this type")
              }
              if let Some(outer) = outer_name {
                d.remark(
                  outer,
                  format_args!("declared within this {inside_of}"),
                );
              }
            }
          } else if inside.is_some_and(|k| k != syn::DeclKind::Enum) {
            let mut d =
              report.error(format_args!("{inside_of} fields must have types"));
            if let Some(name) = name {
              d = d.saying(name, "expected type")
            }
            if let Some(outer) = outer_name {
              d.remark(outer, format_args!("declared within this {inside_of}"));
            }
          }

          parse_wheres(pz, toks, report, &mut attrs);

          if inside.is_none() {
            report
              .error("bare fields are not permitted at the file level")
              .at(name?);
          }

          Some(What::Field(syn::Field {
            name: name?,
            number,
            ty,
            attrs: Vec::new(),
          }))
        },
      )
      .case(Lexeme::eof(), |_, _| {
        // TODO: handle unmoored attributes?
        None
      })
      .take(toks, report)??;

    match done {
      What::Attr(attr) => attrs.push(attr),
      What::Import(mut import) => {
        import.attrs = mem::take(&mut attrs);
        match &mut imports {
          Some(imports) => imports.push(import),
          None => {
            report
              .error("imports are only permitted at the top of the file")
              .at(&import);
          }
        }
      }
      What::Decl(mut decl) => {
        decl.attrs = attrs;
        return Some(syn::Item::Decl(decl));
      }
      What::Field(mut field) => {
        field.attrs = attrs;
        return Some(syn::Item::Field(field));
      }
    }
  }
}

pub fn lex<'t>(
  file: ilex::File<'t>,
  report: &ilex::Report,
) -> Result<token::Stream<'t>, ilex::Fatal> {
  file.lex(Pz::get().spec(), report)
}

pub fn parse<'t>(
  stream: &'t token::Stream<'t>,
  report: &ilex::Report,
) -> Result<syn::PzFile<'t>, ilex::Fatal> {
  let _u = stream.context().use_for_debugging_spans();
  let mut cursor = stream.cursor();

  let pz = Pz::get();
  let pkg = parse_package(pz, &mut cursor, report);

  let mut imports = Vec::new();
  let mut items = Vec::new();
  while !cursor.is_empty() {
    let item = parse_item(
      pz,
      &mut cursor,
      report,
      // Only parse imports before we parse any other kind of item.
      items.is_empty().then_some(&mut imports),
      None,
      None,
    );
    if let Some(item) = item {
      items.push(item);
    }
  }

  let (attrs, package) = pkg.unwrap();
  Ok(syn::PzFile {
    attrs,
    package,
    imports,
    items,
  })
}
