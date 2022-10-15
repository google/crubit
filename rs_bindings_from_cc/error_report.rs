// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::Serialize;

#[doc(hidden)]
pub mod macro_internal {
    use std::borrow::Cow;
    use std::fmt::{self, Arguments, Display, Formatter};

    pub use anyhow;
    pub use arc_anyhow;
    pub use std::format_args;

    /// An error that stores its format string as well as the formatted message.
    #[derive(Debug, Clone)]
    pub struct AttributedError {
        pub fmt: Cow<'static, str>,
        pub message: Cow<'static, str>,
    }

    impl AttributedError {
        pub fn new_static(fmt: &'static str, args: Arguments) -> arc_anyhow::Error {
            arc_anyhow::Error::from(anyhow::Error::from(match args.as_str() {
                // This format string has no parameters.
                Some(s) => Self { fmt: Cow::Borrowed(s), message: Cow::Borrowed(s) },
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

    impl Display for AttributedError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for AttributedError {}
}

use crate::macro_internal::AttributedError;

/// Evaluates to an [`arc_anyhow::Error`].
///
/// Otherwise similar to [`anyhow::anyhow`].
#[macro_export]
macro_rules! anyhow {
    ($fmt:literal $(,)?) => {
        $crate::macro_internal::AttributedError::new_static(
            $fmt,
            $crate::macro_internal::format_args!($fmt),
        )
    };
    ($err:expr $(,)?) => {
        $crate::macro_internal::AttributedError::new_dynamic($err)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::macro_internal::AttributedError::new_static(
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

pub trait ErrorReporting {
    fn insert(&mut self, error: &arc_anyhow::Error);
    fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>>;
}

/// A null [`ErrorReporting`] strategy.
pub struct IgnoreErrors;

impl ErrorReporting for IgnoreErrors {
    fn insert(&mut self, _error: &arc_anyhow::Error) {}

    fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>> {
        Ok(vec![])
    }
}

/// An aggregate of zero or more errors.
#[derive(Default, Serialize)]
pub struct ErrorReport {
    #[serde(flatten)]
    map: BTreeMap<Cow<'static, str>, ErrorReportEntry>,
}

impl ErrorReport {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ErrorReporting for ErrorReport {
    fn insert(&mut self, error: &arc_anyhow::Error) {
        if let Some(error) = error.downcast_ref::<AttributedError>() {
            let sample_message = if error.message != error.fmt { &*error.message } else { "" };
            self.map.entry(error.fmt.clone()).or_default().add(Cow::Borrowed(sample_message));
        } else {
            self.map.entry(Cow::Borrowed("{}")).or_default().add(Cow::Owned(format!("{error}")));
        }
    }

    fn serialize_to_vec(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}

#[derive(Default, Serialize)]
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
    use std::borrow::Cow;

    #[test]
    fn anyhow_1arg_static_plain() {
        let arc_err = anyhow!("abc");
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abc");
    }

    #[test]
    fn anyhow_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = anyhow!("abc{some_var}");
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn anyhow_1arg_dynamic() {
        let arc_err = anyhow!(format!("abc{}", "def"));
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn anyhow_2arg() {
        let arc_err = anyhow!("abc{}", "def");
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn bail_1arg_static_plain() {
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc") })().unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abc");
    }

    #[test]
    fn bail_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc{some_var}") })().unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn bail_1arg_dynamic() {
        let arc_err =
            (|| -> arc_anyhow::Result<()> { bail!(format!("abc{}", "def")) })().unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn bail_2arg() {
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc{}", "def") })().unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn ensure_pass() {
        let f = || {
            ensure!(true, "unused message");
            Ok(())
        };
        f().unwrap();
    }

    #[test]
    fn ensure_fail_1arg_static_plain() {
        let arc_err = (|| {
            ensure!(false, "abc");
            Ok(())
        })()
        .unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.message, Cow::Borrowed(_)));
        assert_eq!(err.message, "abc");
    }

    #[test]
    fn ensure_fail_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = (|| {
            ensure!(false, "abc{some_var}");
            Ok(())
        })()
        .unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn ensure_fail_1arg_dynamic() {
        let arc_err = (|| {
            ensure!(false, format!("abc{}", "def"));
            Ok(())
        })()
        .unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn ensure_fail_2arg() {
        let arc_err = (|| {
            ensure!(false, "abc{}", "def");
            Ok(())
        })()
        .unwrap_err();
        let err: &AttributedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.message, Cow::Owned(_)));
        assert_eq!(err.message, "abcdef");
    }

    #[test]
    fn error_report() {
        let mut report = ErrorReport::new();
        report.insert(&anyhow!("abc{}", "def"));
        report.insert(&anyhow!("abc{}", "123"));
        report.insert(&anyhow!("error code: {}", 65535));
        report.insert(&anyhow!("no parameters"));
        report.insert(&anyhow!("no parameters"));
        report.insert(&anyhow!("no parameters"));
        report.insert(&anyhow::Error::msg("not attributed").into());

        assert_eq!(
            serde_json::to_string_pretty(&report).unwrap(),
            r#"{
  "abc{}": {
    "count": 2,
    "sample_message": "abcdef"
  },
  "error code: {}": {
    "count": 1,
    "sample_message": "error code: 65535"
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
