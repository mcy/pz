use std::fmt;

use googletest::elements_are;
use googletest::matcher;
use googletest::matchers::empty;
use googletest::matchers::eq;
use googletest::matches_pattern;
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

  verify_that!(
    ast,
    matches_pattern!(syn::PzFile {
      edition: matches_pattern!(syn::Edition {
        value: Text(&ast.ctx, eq("\"2023\"")),
      }),
      package: matches_pattern!(syn::Package {
        path: matches_pattern!(syn::Path {
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
        // TODO
      }
      enum Bar {}
    "#,
  );
  let mut ctx = syn::Context::new(&f);
  let ast = parse(&mut ctx);

  verify_that!(
    ast,
    matches_pattern!(syn::PzFile {
      edition: matches_pattern!(syn::Edition {
        value: Text(&ast.ctx, eq("\"2023\"")),
      }),
      package: matches_pattern!(syn::Package {
        path: matches_pattern!(syn::Path {
          components: elements_are![
            Text(&ast.ctx, eq("foo")),
            Text(&ast.ctx, eq("bar"))
          ],
        }),
      }),
      items: elements_are![
        matches_pattern!(syn::Item::Message(matches_pattern!(syn::Message {
          name: Text(&ast.ctx, eq("Foo"))
        }))),
        matches_pattern!(syn::Item::Enum(matches_pattern!(syn::Enum {
          name: Text(&ast.ctx, eq("Bar"))
        }))),
      ],
    })
  )?;

  Ok(())
}
