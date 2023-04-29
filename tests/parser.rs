use std::fmt;

use googletest::elements_are;
use googletest::field;
use googletest::matcher;
use googletest::matchers::anything;
use googletest::matchers::empty;
use googletest::matchers::eq;
use googletest::matchers::none;
use googletest::matchers::predicate;
use googletest::matchers::some;
use googletest::pat;
use googletest::verify_that;
use googletest::Result;

use pz::syn;

fn file(text: &str) -> pz::pz::File {
  pz::pz::File {
    path: Some("test.pz".to_string()),
    text: Some(unindent::unindent(text).to_string()),
  }
}

fn print_errors(ctx: &mut syn::Context) -> bool {
  ctx
    .report()
    .render(
      &pz::report::RenderOptions {
        show_report_locations: true,
        color: false,
      },
      &mut std::io::stderr(),
    )
    .unwrap()
}

fn parse<'c, 'f>(ctx: &'c mut syn::Context<'f>) -> syn::PzFile<'c, 'f> {
  match syn::PzFile::parse(ctx) {
    Ok(mut file) => {
      assert!(!print_errors(&mut file.ctx));
      file
    }
    Err(ctx) => {
      assert!(!print_errors(ctx));
      unreachable!()
    }
  }
}

struct Text<'f, 'c, M>(&'f syn::Context<'c>, M);
impl<M, S> matcher::Matcher<S> for Text<'_, '_, M>
where
  M: for<'a> matcher::Matcher<&'a str>,
  S: syn::Spanned + fmt::Debug,
{
  fn matches(&self, actual: &S) -> matcher::MatcherResult {
    self.1.matches(&actual.span().text(self.0))
  }

  fn describe(&self, result: matcher::MatcherResult) -> String {
    match result {
      matcher::MatcherResult::Matches => {
        format!("has a value whose span.text() {}", self.1.describe(result))
      }
      matcher::MatcherResult::DoesNotMatch => {
        format!(
          "does not have a value whose span.text() {}",
          self.1.describe(result)
        )
      }
    }
  }
}

#[test]
fn empty_package() -> Result<()> {
  let f = file(
    r#"
      edition = "2023";
      package;
    "#,
  );
  let mut ctx = syn::Context::new(&f);
  let ast = parse(&mut ctx);

  let _p = ast.ctx.enable_printing();
  verify_that!(
    ast,
    pat!(syn::PzFile {
      edition: pat!(syn::Edition {
        value: Text(&ast.ctx, eq("\"2023\"")),
      }),
      package: pat!(syn::Package {
        path: pat!(syn::Path {
          components: empty(),
        }),
      }),
    })
  )?;

  Ok(())
}

#[test]
fn smoke() -> Result<()> {
  let f = file(
    r#"
      edition = "2023";
      package foo.bar;

      message Foo {
        message Baz { value/1: bytes }

        foo/1: i32,
        bar/2: repeated string,
        baz/1000: Baz,

        bonk/-1: repeated foreign.Type,
      }
      enum Bar {}
    "#,
  );
  let mut ctx = syn::Context::new(&f);
  let ast = parse(&mut ctx);

  let _p = ast.ctx.enable_printing();
  verify_that!(
    ast,
    pat!(syn::PzFile {
      edition: pat!(syn::Edition {
        value: Text(&ast.ctx, eq("\"2023\"")),
      }),
      package: pat!(syn::Package {
        path: pat!(syn::Path {
          components: elements_are![
            Text(&ast.ctx, eq("foo")),
            Text(&ast.ctx, eq("bar"))
          ],
        }),
      }),
      items: elements_are![
        pat!(syn::Item::Message(pat!(syn::Message {
          name: Text(&ast.ctx, eq("Foo")),
          items: elements_are![
            pat!(syn::MessageItem::Item(pat!(syn::Item::Message(pat!(
              syn::Message {
                name: Text(&ast.ctx, eq("Baz")),
                items: elements_are![pat!(syn::MessageItem::Field(pat!(
                  syn::Field {
                    name: Text(&ast.ctx, eq("value")),
                    number: some(pat!(syn::IntLit {
                      value(): eq(1),
                    })),
                    ty: pat!(syn::Type {
                      repeated: none(),
                      kind: predicate(|x| matches!(x, syn::TypeKind::Bytes)),
                    }),
                  }
                ))),],
              }
            ))))),
            pat!(syn::MessageItem::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("foo")),
              number: some(pat!(syn::IntLit {
                value(): eq(1),
              })),
              ty: pat!(syn::Type {
                repeated: none(),
                kind: predicate(|x| matches!(x, syn::TypeKind::I32)),
              }),
            }))),
            pat!(syn::MessageItem::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("bar")),
              number: some(pat!(syn::IntLit {
                value(): eq(2),
              })),
              ty: pat!(syn::Type {
                repeated: some(anything()),
                kind: predicate(|x| matches!(x, syn::TypeKind::String)),
              }),
            }))),
            pat!(syn::MessageItem::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("baz")),
              number: some(pat!(syn::IntLit {
                value(): eq(1000),
              })),
              ty: pat!(syn::Type {
                repeated: none(),
                kind: pat!(syn::TypeKind::Path(pat!(syn::Path {
                  components: elements_are![Text(&ast.ctx, eq("Baz")),],
                }))),
              }),
            }))),
            pat!(syn::MessageItem::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("bonk")),
              number: some(pat!(syn::IntLit {
                value(): eq(-1),
              })),
              ty: pat!(syn::Type {
                repeated: some(anything()),
                kind: pat!(syn::TypeKind::Path(pat!(syn::Path {
                  components: elements_are![
                    Text(&ast.ctx, eq("foreign")),
                    Text(&ast.ctx, eq("Type")),
                  ],
                }))),
              }),
            }))),
          ],
        }))),
        pat!(syn::Item::Enum(pat!(syn::Enum {
          name: Text(&ast.ctx, eq("Bar"))
        }))),
      ],
    })
  )?;

  Ok(())
}
