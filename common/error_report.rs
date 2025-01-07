// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use regex::Regex;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::{self, Arguments, Display, Formatter};
use std::rc::Rc;

use serde::Serialize;

#[doc(hidden)]
pub mod macro_internal {
    pub use anyhow;
    pub use arc_anyhow;
    pub use std::format_args;
}

fn to_string_with_cause(error: &arc_anyhow::Error) -> String {
    format!("{:#}", error)
}

fn errors_to_string(errors: &[arc_anyhow::Error]) -> String {
    errors.iter().map(to_string_with_cause).collect::<Vec<_>>().join("\n")
}

/// A list of errors that, when converted to an `anyhow::Error`, can still be
/// individually reported when used with `ErrorReport`.
#[derive(Debug, Clone)]
pub struct ErrorList {
    errors: Vec<arc_anyhow::Error>,
}

impl From<Vec<arc_anyhow::Error>> for ErrorList {
    fn from(errors: Vec<arc_anyhow::Error>) -> Self {
        Self { errors }
    }
}

impl Display for ErrorList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&errors_to_string(&self.errors))
    }
}

impl std::error::Error for ErrorList {}

/// An error that stores its format string as well as the formatted message.
#[derive(Debug, Clone)]
pub struct FormattedError {
    pub fmt: Cow<'static, str>,
    pub message: Cow<'static, str>,
}

impl FormattedError {
    pub fn new_static(fmt: &'static str, args: Arguments) -> arc_anyhow::Error {
        arc_anyhow::Error::from(anyhow::Error::from(match args.as_str() {
            // This format string has no parameters to format at runtime.
            // Note: The compiler can perform optimizations to return `Some`, when even when
            // `fmt` contains placeholders, so we store `fmt` instead of `s` for `fmt` field.
            Some(s) => Self { fmt: Cow::Borrowed(fmt), message: Cow::Borrowed(s) },
            // This format string has parameters and must be formatted.
            None => Self { fmt: Cow::Borrowed(fmt), message: Cow::Owned(fmt::format(args)) },
        }))
    }

    pub fn new_dynamic(err: impl Display) -> arc_anyhow::Error {
        // Use the whole error as the format string. This is preferable to
        // grouping all dynamic errors under the "{}" format string.
        let message = format!("{}", err);
        arc_anyhow::Error::from(anyhow::Error::from(Self {
            fmt: Cow::Owned(message.clone()),
            message: Cow::Owned(message),
        }))
    }
}

impl Display for FormattedError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FormattedError {}

/// Evaluates to an [`arc_anyhow::Error`].
///
/// Otherwise similar to [`anyhow::anyhow`].
#[macro_export]
macro_rules! anyhow {
    ($fmt:literal $(,)?) => {
        $crate::FormattedError::new_static(
            $fmt,
            $crate::macro_internal::format_args!($fmt),
        )
    };
    ($err:expr $(,)?) => {
        $crate::FormattedError::new_dynamic($err)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::FormattedError::new_static(
            $fmt,
            $crate::macro_internal::format_args!($fmt, $($arg)*),
        )
    };
}

/// Returns a [`Result::Err`] containing an [`arc_anyhow::Error`].
///
/// Otherwise similar to [`anyhow::bail`].
#[macro_export]
macro_rules! bail {
    ($fmt:literal $(,)?) => {
        return Err($crate::anyhow!($fmt))
    };
    ($err:expr $(,)?) => {
        return Err($crate::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::anyhow!($fmt, $($arg)*))
    };
}

/// Returns a [`Result::Err`] containing an [`arc_anyhow::Error`] if the given
/// condition evaluates to false.
///
/// Otherwise similar to [`anyhow::ensure`].
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $fmt:literal $(,)?) => {
        if !$cond { $crate::bail!($fmt); }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond { $crate::bail!($err); }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond { $crate::bail!($fmt, $($arg)*); }
    };
}

/// An interface by which errors can be recorded to generate a structured
/// report.
pub trait ErrorReporting {
    fn report(&self, error: &arc_anyhow::Error);
}

pub struct IgnoreErrors;

impl ErrorReporting for IgnoreErrors {
    fn report(&self, _: &arc_anyhow::Error) {}
}

fn hide_unstable_details(input: &str) -> String {
    // Remove line:column in def id
    let regex = Regex::new(r"DefId\(\d+:\d+ ~ ").unwrap();
    let res = regex.replace_all(input, "DefId(").to_string();

    // Remove all hash id in the def id
    let regex = Regex::new(r"\[[0-9a-fA-F]{4}\]").unwrap();
    regex.replace_all(res.as_str(), "").to_string()
}

/// An aggregate of zero or more errors.
#[derive(Default, Debug)]
pub struct ErrorReport {
    // The interior mutability / borrow_mut will never panic: it is never borrowed for longer than
    // a method call, and the methods do not call each other.
    map: RefCell<BTreeMap<Cow<'static, str>, ErrorReportEntry>>,
}

impl ErrorReport {
    pub fn new() -> Self {
        Self::default()
    }

    /// If `enable` is true, returns a pair of an `ErrorReport` and a `dyn
    /// ErrorReporting` that will report errors into the `ErrorReport`.
    ///
    /// If `enable` is false, returns `None` with a `dyn ErrorReporting` that
    /// will ignore errors.
    pub fn new_rc_or_ignore(enable: bool) -> (Option<Rc<Self>>, Rc<dyn ErrorReporting>) {
        if enable {
            let this = Rc::new(Self::default());
            (Some(this.clone()), this)
        } else {
            (None, Rc::new(IgnoreErrors))
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&*self.map.borrow())
            .expect("ErrorReporting serialization to JSON failed unexpectedly")
    }
}

impl ErrorReporting for ErrorReport {
    fn report(&self, error: &arc_anyhow::Error) {
        let root_cause = error.root_cause();
        if let Some(error) = root_cause.downcast_ref::<FormattedError>() {
            let sample_message = if error.message != error.fmt { &*error.message } else { "" };
            self.map
                .borrow_mut()
                .entry(error.fmt.clone())
                .or_default()
                .add(Cow::Owned(hide_unstable_details(sample_message)));
        } else if let Some(error) = root_cause.downcast_ref::<ErrorList>() {
            for error in &error.errors {
                self.report(error);
            }
        } else {
            self.map
                .borrow_mut()
                .entry(Cow::Borrowed("{}"))
                .or_default()
                .add(Cow::Owned(hide_unstable_details(&format!("{error}"))));
        }
    }
}

#[derive(Default, Debug, Serialize)]
struct ErrorReportEntry {
    count: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    sample_message: String,
}

impl ErrorReportEntry {
    fn add(&mut self, message: Cow<str>) {
        if self.count == 0 {
            self.sample_message = message.into_owned();
        }
        self.count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    use arc_anyhow::Result;

    #[gtest]
    fn anyhow_1arg_static_plain() {
        let arc_err = anyhow!("abc");
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abc");
    }

    #[gtest]
    fn anyhow_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = anyhow!("abc{some_var}");
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn anyhow_1arg_dynamic() {
        let arc_err = anyhow!(format!("abc{}", "def"));
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn anyhow_2arg() {
        let arc_err = anyhow!("abc{}", "def");
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn bail_1arg_static_plain() {
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc") })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abc");
    }

    #[gtest]
    fn bail_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc{some_var}") })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn bail_1arg_dynamic() {
        let arc_err =
            (|| -> arc_anyhow::Result<()> { bail!(format!("abc{}", "def")) })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn bail_2arg() {
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc{}", "def") })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn ensure_pass() {
        let f = || {
            ensure!(true, "unused message");
            Ok(())
        };
        f().unwrap();
    }

    #[gtest]
    fn ensure_fail_1arg_static_plain() {
        let arc_err = (|| {
            ensure!(false, "abc");
            Ok(())
        })()
        .unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abc");
    }

    #[gtest]
    fn ensure_fail_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = (|| {
            ensure!(false, "abc{some_var}");
            Ok(())
        })()
        .unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn ensure_fail_1arg_dynamic() {
        let arc_err = (|| {
            ensure!(false, format!("abc{}", "def"));
            Ok(())
        })()
        .unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn ensure_fail_2arg() {
        let arc_err = (|| {
            ensure!(false, "abc{}", "def");
            Ok(())
        })()
        .unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[gtest]
    fn error_report() {
        let report = ErrorReport::new();
        report.report(&anyhow!("abc{}", "def"));
        report.report(&anyhow!("abc{}", "123"));
        report.report(&anyhow!("error code: {}", 65535));
        report.report(&anyhow!("no parameters"));
        report.report(&anyhow!("no parameters"));
        report.report(&anyhow!("no parameters"));
        report.report(&anyhow::Error::msg("not attributed").into());
        report.report(&anyhow!("has context from arc_anyhow::context()").context("the context"));
        report.report(
            &arc_anyhow::Context::context(
                Result::<(), _>::Err(anyhow!("has context from arc_anyhow::Context::context()")),
                "the context",
            )
            .unwrap_err(),
        );
        report.report(
            &arc_anyhow::Context::with_context(
                Result::<(), _>::Err(anyhow!(
                    "has context from arc_anyhow::Context::with_context()"
                )),
                || "the context",
            )
            .unwrap_err(),
        );
        report.report(
            &anyhow!("has three layers of context")
                .context("context 1")
                .context("context 2")
                .context("context 3"),
        );

        expect_eq!(
            serde_json::from_str::<serde_json::Value>(&report.to_json_string()).unwrap(),
            serde_json::json!({
              "abc{}": {
                "count": 2,
                "sample_message": "abcdef"
              },
              "error code: {}": {
                "count": 1,
                "sample_message": "error code: 65535"
              },
              "has context from arc_anyhow::Context::context()": {
                "count": 1
              },
              "has context from arc_anyhow::Context::with_context()": {
                "count": 1
              },
              "has context from arc_anyhow::context()": {
                "count": 1
              },
              "has three layers of context": {
                "count": 1
              },
              "no parameters": {
                "count": 3
              },
              "{}": {
                "count": 1,
                "sample_message": "not attributed"
              }
            }),
        );
    }

    #[gtest]
    fn test_error_list_elements_are_reported() {
        let report = ErrorReport::new();
        report.report(&arc_anyhow::Error::from(ErrorList::from(vec![
            anyhow!("abc{}", "def"),
            anyhow!("hijk"),
        ])));
        expect_eq!(
            serde_json::from_str::<serde_json::Value>(&report.to_json_string()).unwrap(),
            serde_json::json!({
              "abc{}": {
                "count": 1,
                "sample_message": "abcdef"
              },
              "hijk": {
                "count": 1
              }
            }),
        );
    }
}
