// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use regex::Regex;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::{self, Arguments, Display, Formatter};
use std::rc::Rc;

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedError {
    #[serde(default, skip_serializing_if = "str::is_empty")]
    fmt: Cow<'static, str>,
    /// The full error message.
    ///
    /// If this is identical to the `fmt` value, this will be empty.
    #[serde(default, skip_serializing_if = "str::is_empty")]
    full_error: Cow<'static, str>,
}

impl FormattedError {
    pub fn new(fmt: Cow<'static, str>, mut full_error: Cow<'static, str>) -> Self {
        if full_error == fmt {
            full_error = Cow::Borrowed("")
        }
        Self { fmt, full_error }
    }

    pub fn new_static(fmt: &'static str, args: Arguments) -> arc_anyhow::Error {
        arc_anyhow::Error::from(anyhow::Error::from(match args.as_str() {
            // Either the format string has no parameters, or it has parameters which were elided
            // at compile-time. We may still need to store the full_error.
            Some(s) => Self::new(Cow::Borrowed(fmt), Cow::Borrowed(s)),
            // This format string has parameters and must be formatted.
            None => Self::new(Cow::Borrowed(fmt), Cow::Owned(fmt::format(args))),
        }))
    }

    pub fn new_dynamic(err: impl Display) -> arc_anyhow::Error {
        // Use the whole error as the format string. This is preferable to
        // grouping all dynamic errors under the "{}" format string.
        arc_anyhow::Error::from(anyhow::Error::from(Self::new(
            Cow::Owned(err.to_string()),
            Cow::Borrowed(""),
        )))
    }

    pub fn fmt(&self) -> &str {
        &self.fmt
    }

    pub fn full_error(&self) -> &str {
        &self.full_error
    }
}

impl Display for FormattedError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.full_error.is_empty() {
            write!(f, "{}", self.fmt)
        } else {
            write!(f, "{}", self.full_error)
        }
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
            #[allow(clippy::literal_string_with_formatting_args)] $fmt,
            $crate::macro_internal::format_args!($fmt),
        )
    };
    ($err:expr $(,)?) => {
        $crate::FormattedError::new_dynamic($err)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::FormattedError::new_static(
            #[allow(clippy::literal_string_with_formatting_args)] $fmt,
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

    /// Enter `item`, and return the item to replace with upon exiting.
    fn enter_item(&self, item: ItemName) -> Option<ItemName>;
    /// Assert that we are currently in `item`.
    fn assert_in_item(&self, item: ItemName);
    /// Exit `item`, and restore the scope to `replace_with`.
    fn exit_item(&self, item: ItemName, replace_with: Option<ItemName>);

    /// Adds the provided category metadata bits to the current item.
    fn add_category(&self, category: Category);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Category {
    /// This item is a function.
    Function = 1 << 0,

    /// This item is a global variable.
    Variable = 1 << 1,

    /// This item is a type definition.
    Type = 1 << 2,

    /// This item is a type alias.
    Alias = 1 << 3,

    /// This item is a namespace (C++) or module (Rust).
    Namespace = 1 << 4,

    /// This item is a generic or template instantiation.
    ///
    /// For example, with `template <typename T> class MyTemplate{}`, `MyTemplate<int>` is a
    /// `GenericInstantiation`.
    GenericInstantiation = 1 << 6,

    /// This item is a non-movable type in the language it's receiving bindings for.
    ///
    /// If this is a C++ type, then NonMovable is set when the resulting Rust
    /// type is pinned in memory and can't be used by value without ctor. If it is a Rust type,
    /// then NonMovable is set when the corresponding C++ type doesn't have a move constructor.
    NonMovable = 1 << 7,

    /// This item is a bridged type, which cannot be used by value.
    Bridge = 1 << 8,

    /// This is an `unsafe` item. If it is a type, then it's a type that causes Crubit to
    /// mark any function accepting it as unsafe.
    ///
    /// As an example, the C++ items `struct Foo{ T* x};` and `void Foo(T* x);` are unsafe.
    Unsafe = 1 << 9,
    // TODO(b/468093766): Abstract? base classes, public inheritance
}

/// A named unique identifier for an item.
#[derive(Clone, PartialEq, Debug)]
pub struct ItemName {
    /// The human-readable qualified name of the item.
    pub name: Rc<str>,
    /// Unique ID per target. (E.g. the address of the AST node.)
    pub id: u64,
    // A unique name for log aggregation purposes. For C++ items, this is a clang Unified Symbol
    // Resolution (USR) string.
    pub unique_name: Option<Rc<str>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceLanguage {
    Cpp,
    Rust,
}

pub struct ItemScope<'a> {
    report: &'a dyn ErrorReporting,
    item: ItemName,
    old_item: Option<ItemName>,
}

impl<'a> ItemScope<'a> {
    pub fn new(report: &'a dyn ErrorReporting, item: ItemName) -> Self {
        let old_item = report.enter_item(item.clone());
        Self { report, item, old_item }
    }
}

impl Drop for ItemScope<'_> {
    fn drop(&mut self) {
        self.report.exit_item(self.item.clone(), std::mem::take(&mut self.old_item));
    }
}

pub struct IgnoreErrors;

impl ErrorReporting for IgnoreErrors {
    fn report(&self, _: &arc_anyhow::Error) {}
    fn enter_item(&self, _: ItemName) -> Option<ItemName> {
        None
    }
    fn assert_in_item(&self, _: ItemName) {}
    fn exit_item(&self, _: ItemName, _: Option<ItemName>) {}
    fn add_category(&self, _: Category) {}
}

fn hide_unstable_details(input: &str) -> String {
    // Remove line:column in def id
    let regex = Regex::new(r"DefId\(\d+:\d+ ~ ").unwrap();
    let res = regex.replace_all(input, "DefId(").to_string();

    // Remove all hash id in the def id
    let regex = Regex::new(r"\[[0-9a-fA-F]{4}\]").unwrap();
    regex.replace_all(res.as_str(), "").to_string()
}

/// Errors per-item.
#[derive(Debug)]
pub struct ErrorReport {
    /// The language where items are declared.
    source_language: SourceLanguage,
    // The interior mutability / borrow_mut will never panic: it is never borrowed for longer than
    // a method call, and the methods do not call each other.
    map: RefCell<BTreeMap<u64, ErrorReportEntry>>,
    // TODO(jeanpierreda): This should really be passed around rather than mutated in the
    // BindingsGenerator. For example, if we used a totally separate `BindingsGenerator`
    // which is the same as the old one except that it has a different input.
    current_item: RefCell<Option<ItemName>>,
}

struct SerializeIterator<It: Clone + Iterator>(It);

impl<It: Clone + Iterator> Serialize for SerializeIterator<It>
where
    It::Item: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for e in self.0.clone() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

impl ErrorReport {
    pub fn new(source_language: SourceLanguage) -> Self {
        Self { source_language, map: Default::default(), current_item: Default::default() }
    }

    /// If `enable` is true, returns a pair of an `ErrorReport` and a `dyn
    /// ErrorReporting` that will report errors into the `ErrorReport`.
    ///
    /// If `enable` is false, returns `None` with a `dyn ErrorReporting` that
    /// will ignore errors.
    pub fn new_rc_or_ignore(
        enable: bool,
        source_language: SourceLanguage,
    ) -> (Option<Rc<Self>>, Rc<dyn ErrorReporting>) {
        if enable {
            let this = Rc::new(Self::new(source_language));
            (Some(this.clone()), this)
        } else {
            (None, Rc::new(IgnoreErrors))
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&SerializeIterator(self.map.borrow().values()))
            .expect("ErrorReporting serialization to JSON failed unexpectedly")
    }

    fn current_item(&self) -> ItemName {
        self.current_item.borrow().clone().unwrap_or_else(|| DEFAULT_ITEM.with(|item| item.clone()))
    }
}

thread_local! {
    static DEFAULT_ITEM: ItemName = ItemName {name: Rc::from(""), id: 0, unique_name: None };
}

impl ErrorReporting for ErrorReport {
    fn report(&self, error: &arc_anyhow::Error) {
        let root_cause = error.root_cause();
        let reported_error;
        if let Some(error) = root_cause.downcast_ref::<FormattedError>() {
            reported_error = Some(FormattedError::new(
                error.fmt.clone(),
                Cow::Owned(hide_unstable_details(&error.full_error)),
            ));
        } else if let Some(error) = root_cause.downcast_ref::<ErrorList>() {
            for error in &error.errors {
                self.report(error);
            }
            reported_error = None;
        } else {
            reported_error = Some(FormattedError::new(
                Cow::Borrowed("{}"),
                Cow::Owned(hide_unstable_details(&format!("{error}"))),
            ));
        }
        if let Some(reported_error) = reported_error {
            self.map
                .borrow_mut()
                .entry(self.current_item().id)
                .or_insert_with(|| ErrorReportEntry {
                    source_language: Some(self.source_language),
                    ..Default::default()
                })
                .errors
                .push(reported_error);
        }
    }

    fn enter_item(&self, item: ItemName) -> Option<ItemName> {
        // At least populate with an empty ErrorReportEntry, so that we can detect error-free items
        // after the fact.
        let mut map = self.map.borrow_mut();
        match map.entry(item.id) {
            Entry::Vacant(e) => {
                e.insert(ErrorReportEntry {
                    source_language: Some(self.source_language),
                    name: item.name.clone(),
                    unique_name: item.unique_name.clone(),
                    ..Default::default()
                });
            }
            Entry::Occupied(e) => {
                assert_eq!(
                    e.get().name,
                    item.name,
                    "distinct items with the same unique ID: {}",
                    item.id
                );
            }
        }
        self.current_item.borrow_mut().replace(item)
    }

    fn assert_in_item(&self, item: ItemName) {
        assert_eq!(
            self.current_item(),
            item,
            "error report failure: not in item scope: {}",
            item.name,
        );
    }

    fn exit_item(&self, item: ItemName, replace_with: Option<ItemName>) {
        let old_item = std::mem::replace(&mut *self.current_item.borrow_mut(), replace_with);
        assert_eq!(
            old_item,
            Some(item),
            "bad scoping: stopped handling an item, but we were processing a different item."
        );
    }

    fn add_category(&self, category: Category) {
        self.map
            .borrow_mut()
            .entry(self.current_item().id)
            .or_insert_with(|| ErrorReportEntry {
                source_language: Some(self.source_language),
                ..Default::default()
            })
            .category |= category as u32;
    }
}

/// An entry in an error report.
///
/// The serialized JSON error report is a sequence of these, so format changes should be kept
/// backwards-compatible.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ErrorReportEntry {
    pub source_language: Option<SourceLanguage>,

    pub name: Rc<str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FormattedError>,

    /// A bitset of Category flags.
    ///
    /// (Note: can't use flagset, because it fails in recent rustc.)
    #[serde(default, skip_serializing_if = "is_default")]
    pub category: u32,

    // A unique name for log aggregation purposes. For C++ items, this is a clang Unified Symbol
    // Resolution (USR) string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<Rc<str>>,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

/// Reporter for fatal errors that will cause bindings generation to fail.
pub trait ReportFatalError {
    /// Reports a fatal error that will cause bindings generation to fail.
    ///
    /// These errors should be issued only in response to misusage of Crubit
    /// itself, such as incorrect use of Crubit-specific annotations.
    fn report(&self, msg: &str);
}

/// A collection of errors that should cause bindings generation to fail.
#[derive(Default)]
pub struct FatalErrors {
    fatal_errors: std::cell::RefCell<String>,
}

impl ReportFatalError for FatalErrors {
    fn report(&self, msg: &str) {
        let mut errors = self.fatal_errors.borrow_mut();
        errors.push('\n');
        errors.push_str(msg);
    }
}

impl FatalErrors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take_string(&self) -> String {
        std::mem::take(&mut *self.fatal_errors.borrow_mut())
    }
}

impl Drop for FatalErrors {
    fn drop(&mut self) {
        let errors = self.fatal_errors.borrow();
        if !errors.is_empty() {
            panic!("Fatal errors in binding generation were not reported:{}", errors);
        }
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
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "");
    }

    #[gtest]
    fn anyhow_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = anyhow!("abc{some_var}");
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.full_error, Cow::Owned(_)));
        assert_eq!(err.full_error, "abcdef");
    }

    #[gtest]
    fn anyhow_1arg_dynamic() {
        let arc_err = anyhow!(format!("abc{}", "def"));
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "");
    }

    #[gtest]
    fn anyhow_2arg() {
        let arc_err = anyhow!("abc{}", "def");
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "abcdef");
    }

    #[gtest]
    fn bail_1arg_static_plain() {
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc") })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc");
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "");
    }

    #[gtest]
    fn bail_1arg_static_fmt() {
        let some_var = "def";
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc{some_var}") })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{some_var}");
        assert!(matches!(err.full_error, Cow::Owned(_)));
        assert_eq!(err.full_error, "abcdef");
    }

    #[gtest]
    fn bail_1arg_dynamic() {
        let arc_err =
            (|| -> arc_anyhow::Result<()> { bail!(format!("abc{}", "def")) })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Owned(_)));
        assert_eq!(err.fmt, "abcdef");
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "");
    }

    #[gtest]
    fn bail_2arg() {
        let arc_err = (|| -> arc_anyhow::Result<()> { bail!("abc{}", "def") })().unwrap_err();
        let err: &FormattedError = arc_err.downcast_ref().unwrap();
        assert!(matches!(err.fmt, Cow::Borrowed(_)));
        assert_eq!(err.fmt, "abc{}");
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "abcdef");
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
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "");
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
        assert!(matches!(err.full_error, Cow::Owned(_)));
        assert_eq!(err.full_error, "abcdef");
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
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "");
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
        assert!(matches!(err.full_error, Cow::Borrowed(_)));
        assert_eq!(err.full_error, "abcdef");
    }

    #[gtest]
    fn error_report() {
        let report = ErrorReport::new(SourceLanguage::Cpp);
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
            serde_json::json!([
                {
                    "source_language": "Cpp",
                    "name": "",
                    "errors": [
                        {
                            "fmt": "abc{}",
                            "full_error": "abcdef",
                        },
                        {
                            "fmt": "abc{}",
                            "full_error": "abc123",
                        },
                        {
                            "fmt": "error code: {}",
                            "full_error": "error code: 65535",
                        },
                        {
                            "fmt": "no parameters",
                        },
                        {
                            "fmt": "no parameters",
                        },
                        {
                            "fmt": "no parameters",
                        },
                        {
                            "fmt": "{}",
                            "full_error": "not attributed",
                        },
                        {
                            "fmt": "has context from arc_anyhow::context()",
                        },
                        {
                            "fmt": "has context from arc_anyhow::Context::context()",
                        },
                        {
                            "fmt": "has context from arc_anyhow::Context::with_context()",
                        },
                        {
                            "fmt": "has three layers of context",
                        },
                    ],
                },
            ]),
        );
    }

    #[gtest]
    fn error_report_item_name() {
        let report = ErrorReport::new(SourceLanguage::Rust);
        {
            let _scope =
                ItemScope::new(&report, ItemName { name: "foo".into(), id: 1, unique_name: None });
            report.report(&anyhow!("error in {}", "item 1"));
        }
        {
            let _scope = ItemScope::new(
                &report,
                ItemName { name: "bar".into(), id: 2, unique_name: Some("abc123".into()) },
            );
            report.report(&anyhow!("error in {}", "item 2"));
        }

        expect_eq!(
            serde_json::from_str::<serde_json::Value>(&report.to_json_string()).unwrap(),
            serde_json::json!([
                {
                    "source_language": "Rust",
                    "name": "foo",
                    "errors": [
                        {
                            "fmt": "error in {}",
                            "full_error": "error in item 1",
                        },
                    ],
                },
                {
                    "source_language": "Rust",
                    "name": "bar",
                    "errors": [
                        {
                            "fmt": "error in {}",
                            "full_error": "error in item 2",
                        },
                    ],
                    "unique_name": "abc123",
                },
            ]),
        );
    }

    #[gtest]
    fn test_error_list_elements_are_reported() {
        let report = ErrorReport::new(SourceLanguage::Cpp);
        report.report(&arc_anyhow::Error::from(ErrorList::from(vec![
            anyhow!("abc{}", "def"),
            anyhow!("hijk"),
        ])));
        expect_eq!(
            serde_json::from_str::<serde_json::Value>(&report.to_json_string()).unwrap(),
            serde_json::json!([
                {
                    "source_language": "Cpp",
                    "name": "",
                    "errors": [
                        {
                            "fmt": "abc{}",
                            "full_error": "abcdef"
                        },
                        {
                            "fmt": "hijk"
                        },
                    ]
                },
            ]),
        );
    }
}
