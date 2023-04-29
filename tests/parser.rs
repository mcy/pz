
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

use pz::report::Report;
use pz::syn;

fn file(text: &str) -> pz::pz::File {
  pz::pz::File {
    path: Some("test.pz".to_string()),
    text: Some(unindent::unindent(text).to_string()),
  }
}

fn print_errors(report: &mut Report, ctx: &syn::Context) -> bool {
  report
    .render(
      ctx,
      &pz::report::RenderOptions {
        show_report_locations: true,
        color: false,
      },
      &mut std::io::stderr(),
    )
    .unwrap()
}

fn parse<'c, 'f>(ctx: &'c mut syn::Context<'f>) -> syn::PzFile<'c, 'f> {
  let mut report = Report::new();
  match syn::PzFile::parse(ctx, &mut report) {
    Ok(mut file) => {
      assert!(!print_errors(&mut report, &mut file.ctx));
      file
    }
    Err(ctx) => {
      assert!(!print_errors(&mut report, ctx));
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
        message Baz { value: bytes = 1 }

        foo: i32 = 1,
        bar: repeated string = 2,
        baz: Baz = 1000,

        bonk: repeated foreign.Type = -1,
      }

      enum Bar {
        FIRST = 1,
        SECOND = 2,
        NEGATIVE = -9999,
      }
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
        pat!(syn::Item::Decl(pat!(syn::Decl {
          name: Text(&ast.ctx, eq("Foo")),
          kind: eq(syn::DeclKind::Message),
          items: elements_are![
            pat!(syn::Item::Decl(pat!(syn::Decl {
              name: Text(&ast.ctx, eq("Baz")),
              kind: eq(syn::DeclKind::Message),
              items: elements_are![pat!(syn::Item::Field(pat!(syn::Field {
                name: Text(&ast.ctx, eq("value")),
                number: some(pat!(syn::IntLit {
                  value(): eq(1),
                })),
                ty: some(pat!(syn::Type {
                  repeated: none(),
                  kind: predicate(|x| matches!(x, syn::TypeKind::Bytes)),
                })),
              }))),],
            }))),
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("foo")),
              number: some(pat!(syn::IntLit {
                value(): eq(1),
              })),
              ty: some(pat!(syn::Type {
                repeated: none(),
                kind: predicate(|x| matches!(x, syn::TypeKind::I32)),
              })),
            }))),
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("bar")),
              number: some(pat!(syn::IntLit {
                value(): eq(2),
              })),
              ty: some(pat!(syn::Type {
                repeated: some(anything()),
                kind: predicate(|x| matches!(x, syn::TypeKind::String)),
              })),
            }))),
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("baz")),
              number: some(pat!(syn::IntLit {
                value(): eq(1000),
              })),
              ty: some(pat!(syn::Type {
                repeated: none(),
                kind: pat!(syn::TypeKind::Path(pat!(syn::Path {
                  components: elements_are![Text(&ast.ctx, eq("Baz")),],
                }))),
              })),
            }))),
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("bonk")),
              number: some(pat!(syn::IntLit {
                value(): eq(-1),
              })),
              ty: some(pat!(syn::Type {
                repeated: some(anything()),
                kind: pat!(syn::TypeKind::Path(pat!(syn::Path {
                  components: elements_are![
                    Text(&ast.ctx, eq("foreign")),
                    Text(&ast.ctx, eq("Type")),
                  ],
                }))),
              })),
            }))),
          ],
        }))),
        pat!(syn::Item::Decl(pat!(syn::Decl {
          name: Text(&ast.ctx, eq("Bar")),
          kind: eq(syn::DeclKind::Enum),
          items: elements_are![
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("FIRST")),
              number: some(pat!(syn::IntLit {
                value(): eq(1),
              })),
              ty: none(),
            }))),
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("SECOND")),
              number: some(pat!(syn::IntLit {
                value(): eq(2),
              })),
              ty: none(),
            }))),
            pat!(syn::Item::Field(pat!(syn::Field {
              name: Text(&ast.ctx, eq("NEGATIVE")),
              number: some(pat!(syn::IntLit {
                value(): eq(-9999),
              })),
              ty: none(),
            }))),
          ],
        }))),
      ],
    })
  )?;

  Ok(())
}
