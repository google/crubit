// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use regex::Regex;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::{self, Arguments, Display, Formatter};

use serde::Serialize;

#[doc(hidden)]
pub mod macro_internal {

    pub use anyhow;
    pub use arc_anyhow;
    pub use std::format_args;
}

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
        if !$cond { bail!($fmt); }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond { bail!($err); }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond { bail!($fmt, $($arg)*); }
    };
}

pub trait ErrorReporting: std::fmt::Debug {
    /// Inserts a new error. Uses interior mutability so that references can be
    /// shared freely.
    fn insert(&self, error: &arc_anyhow::Error);
    fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>>;
    fn serialize_to_string(&self) -> anyhow::Result<String>;
}

/// A null [`ErrorReporting`] strategy.
#[derive(Debug)]
pub struct IgnoreErrors;

impl ErrorReporting for IgnoreErrors {
    fn insert(&self, _error: &arc_anyhow::Error) {}

    fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>> {
        Ok(vec![])
    }

    fn serialize_to_string(&self) -> anyhow::Result<String> {
        Ok(String::new())
    }
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
}

impl ErrorReporting for ErrorReport {
    fn insert(&self, error: &arc_anyhow::Error) {
        let root_cause = error.root_cause();
        if let Some(error) = root_cause.downcast_ref::<FormattedError>() {
            let sample_message = if error.message != error.fmt { &*error.message } else { "" };
            self.map
                .borrow_mut()
                .entry(error.fmt.clone())
                .or_default()
                .add(Cow::Owned(hide_unstable_details(sample_message)));
        } else {
            self.map
                .borrow_mut()
                .entry(Cow::Borrowed("{}"))
                .or_default()
                .add(Cow::Owned(hide_unstable_details(&format!("{error}"))));
        }
    }

    fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(&*self.map.borrow())?)
    }

    fn serialize_to_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(&*self.map.borrow())?)
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
        report.insert(&anyhow!("abc{}", "def"));
        report.insert(&anyhow!("abc{}", "123"));
        report.insert(&anyhow!("error code: {}", 65535));
        report.insert(&anyhow!("no parameters"));
        report.insert(&anyhow!("no parameters"));
        report.insert(&anyhow!("no parameters"));
        report.insert(&anyhow::Error::msg("not attributed").into());
        report.insert(&anyhow!("has context from arc_anyhow::context()").context("the context"));
        report.insert(
            &arc_anyhow::Context::context(
                Result::<(), _>::Err(anyhow!("has context from arc_anyhow::Context::context()")),
                "the context",
            )
            .unwrap_err(),
        );
        report.insert(
            &arc_anyhow::Context::with_context(
                Result::<(), _>::Err(anyhow!(
                    "has context from arc_anyhow::Context::with_context()"
                )),
                || "the context",
            )
            .unwrap_err(),
        );
        report.insert(
            &anyhow!("has three layers of context")
                .context("context 1")
                .context("context 2")
                .context("context 3"),
        );

        assert_eq!(
            report.serialize_to_string().unwrap(),
            r#"{
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
}"#,
        );
    }
}
