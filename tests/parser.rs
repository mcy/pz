use std::fmt;

use googletest::elements_are;
use googletest::matcher;
use googletest::matchers::empty;
use googletest::matchers::eq;
use googletest::matches_pattern;
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

fn parse(file: &pz::pz::File) -> syn::PzFile {
  let mut report = Report::new(file);
  let file = syn::PzFile::parse(file, &mut report);
  let had_errors = report
    .render(
      &pz::report::RenderOptions {
        show_report_locations: true,
        color: true,
      },
      &mut std::io::stderr(),
    )
    .unwrap();

  assert!(!had_errors);
  file.unwrap()
}

struct Text<'f, M>(&'f pz::pz::File, M);
impl<M, S> matcher::Matcher<S> for Text<'_, M>
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

  verify_that!(
    parse(&f),
    matches_pattern!(syn::PzFile {
      edition: matches_pattern!(syn::Edition {
        value: Text(&f, eq("\"2023\"")),
      }),
      package: matches_pattern!(syn::Package {
        components: empty(),
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
    "#,
  );

  verify_that!(
    parse(&f),
    matches_pattern!(syn::PzFile {
      edition: matches_pattern!(syn::Edition {
        value: Text(&f, eq("\"2023\"")),
      }),
      package: matches_pattern!(syn::Package {
        components: elements_are![Text(&f, eq("foo")), Text(&f, eq("bar"))],
      }),
    })
  )?;

  Ok(())
}
