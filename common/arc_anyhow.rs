// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Copyable, equality-comparable `Error`/`Result` types, wrapping `anyhow`.
//!
//! This module is a drop-in replacement for the `anyhow` crate, with the caveat
//! that not all features are implemented, and it may be slower due to requiring
//! an additional layer of boxing every time it round trips back to an
//! `anyhow::Error`.
//!
//! Downcasting by value will never be supported, because of the way copying is
//! implemented -- via `Arc`.
//!
//! ## Salsa
//!
//! The main motivation for this crate is caching return values which may have
//! an error, such as with Salsa. (Although this applies to any approach to
//! caching, not just Salsa specifically.)
//!
//! For example, a memoized function  which returns an `T` on success, and hides
//! this behind an `Rc` for performance, should return
//! `arc_anyhow::Result<Rc<T>>`, not `Rc<anyhow::Result<T>>`. Because
//! `anyhow::Error` cannot be cloned, `Rc<Result<T, anyhow::Error>>` is very
//! nearly useless, as one cannot create a new `Rc<Result<U, anyhow::Error>>`
//! containing the same error. Error propagation with cached errors requires
//! that the underlying error type

use std::fmt::{Debug, Display};
use std::sync::Arc;

/// A clonable, equality-comparable error type, like `anyhow::Error`.
///
/// `Error` is to `anyhow::Error` as `Rc<T>` is to `Box<T>`.
///
/// Two errors are equal if they are identical (i.e. they both have a common
/// cloned-from ancestor.)
#[derive(Clone)]
pub struct Error(Arc<anyhow::Error>);

impl Error {
    /// Convert this into an `anyhow::Error`.
    ///
    /// This creates an extra layer of pointer indirection and should be avoided
    /// when possible.
    pub fn into_anyhow(self) -> anyhow::Error {
        let std_error: StdError = self.into();
        std_error.into()
    }

    /// Gets the lowest level cause of this error.
    ///
    /// Similar to [`anyhow::Error::root_cause`].
    pub fn root_cause(&self) -> &(dyn std::error::Error + 'static) {
        // This crate's `StdError` type is private, so recurse into it to get an
        // error the caller might expect to find.
        let e = self.0.root_cause();
        if let Some(e) = e.downcast_ref::<StdError>() { e.0.root_cause() } else { e }
    }

    pub fn context<C>(self, context: C) -> Self
    where
        C: Display + Send + Sync + 'static,
    {
        self.into_anyhow().context(context).into()
    }

    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.0.downcast_ref()
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self, &*other)
    }
}

impl Eq for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(&*self.0, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Debug::fmt(&*self.0, f)
    }
}

// Note: this blanket impl prevents Error from implementing `std::error::Error`.
// We need all "normal" errors -- say, all those that implemen
// `std::error::Error` -- to be convertible to `Error`, but this would cause
// overlapping impls if `Error` itself implemented `std::error::Error`. For that
// reason, both `anyhow` and `arc_anyhow` don't actually implement the error
// trait!
impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(e: T) -> Self {
        Error(Arc::new(e.into()))
    }
}

/// Helper type which can be converted to `anyhow::Error` directly, by
/// implementing `std::error::Error`.
///
/// `arc_anyhow::Error` cannot itself implement `std::error::Error` for the same
/// reason that `anyhow::Error` cannot -- at various places it produces blanket
/// impl conflicts and other metaprogramming difficulties.
#[derive(Clone)]
struct StdError(Arc<anyhow::Error>);

impl Display for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(&*self.0, f)
    }
}

impl Debug for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Debug::fmt(&*self.0, f)
    }
}

impl std::error::Error for StdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<Error> for StdError {
    fn from(e: Error) -> Self {
        StdError(e.0)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A replacement for anyhow::Context.
///
/// (`anyhow::Context` cannot be used because it is sealed. Even if it weren't
/// sealed, it returns `anyhow::Result` instead of `arc_anyhow::Result`.)
pub trait Context<T, E> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

// Note: can't use `where Result<T, E>: anyhow::Context<T, E>` due to coherence
// rules. The bounds for `anyhow::Context` can change to overlap.
// Instead, we implement `Context` for everything *except* `anyhow::Error`!

impl<T, E> Context<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        anyhow::Context::context(self, context).map_err(|e| e.into())
    }
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        anyhow::Context::with_context(self, f).map_err(|e| e.into())
    }
}

impl<T> Context<T, std::convert::Infallible> for Option<T> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        anyhow::Context::context(self, context).map_err(|e| e.into())
    }
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        anyhow::Context::with_context(self, f).map_err(|e| e.into())
    }
}

impl<T> Context<T, Error> for Result<T, Error> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        let e: Result<T, StdError> = self.map_err(|e| e.into());
        e.context(context)
    }
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        let e: Result<T, StdError> = self.map_err(|e| e.into());
        e.with_context(f)
    }
}

#[doc(hidden)]
pub mod macro_internal {
    pub use ::anyhow;
}

/// An `anyhow` macro which returns an `RcError` instead of an `anyhow::Error`.
#[macro_export]
macro_rules! anyhow {
    ($($t:tt)*) => {
        {
            // specifically constrain to Error, so that type inference works maximally.
            $crate::Error::from($crate::macro_internal::anyhow::anyhow!($($t)*))
        }
    }
}

/// An `ensure` macro which returns an `Result` instead of an
/// `anyhow::Result`.
#[macro_export]
macro_rules! ensure {
    ($($t:tt)*) => {
        {
            let result = || -> $crate::macro_internal::anyhow::Result<()> {
                $crate::macro_internal::anyhow::ensure!($($t)*);
                Ok(())
            }();
            if let Err(e) = result {
                // specifically constrain to Error, so that type inference works maximally.
                return $crate::Result::Err($crate::Error::from(e));
            }
        }
    }
}

/// A `bail` macro which returns an `Result` instead of an
/// `anyhow::Result`.
#[macro_export]
macro_rules! bail {
    ($($t:tt)*) => {
        return $crate::Result::Err($crate::anyhow!($($t)*))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    use super::Result as AAResult;

    #[gtest]
    fn test_result_context() {
        let result: AAResult<()> = AAResult::Err(anyhow!("Something went wrong!"))
            .with_context(|| "context 1")
            .context("context 2");
        let err = result.unwrap_err();

        assert_eq!(&format!("{err}"), "context 2",);
        assert_eq!(&format!("{err:#}"), "context 2: context 1: Something went wrong!",);
        assert_eq!(
            &format!("{err:?}"),
            "context 2\n\n\
                Caused by:\
                \n    0: context 1\
                \n    1: Something went wrong!\
                ",
        );
        // This is probably a bad idea to actually test, for stability reasons,
        // but the Debug representation is also threaded through
        // unchanged:
        //
        // ```
        // assert_eq!(
        //     &format!("{err:#?}"),
        //     "Error {\
        //     \n    context: \"context 2\",\
        //     \n    source: Error {\
        //     \n        context: \"context 1\",\
        //     \n        source: \"Something went wrong!\",\
        //     \n    },\
        //     \n}",
        // );
        // ```
    }
    #[gtest]
    fn test_error_context() {
        let err = anyhow!("Something went wrong!").context("context 1").context("context 2");
        assert_eq!(&format!("{err}"), "context 2",);
        assert_eq!(&format!("{err:#}"), "context 2: context 1: Something went wrong!",);
        assert_eq!(
            &format!("{err:?}"),
            "context 2\n\n\
                Caused by:\
                \n    0: context 1\
                \n    1: Something went wrong!\
                ",
        );
    }

    #[gtest]
    fn test_macro_anyhow() {
        assert_eq!(&format!("{}", anyhow!("message")), "message");
    }

    #[gtest]
    fn test_macro_bail() {
        let err = (|| -> AAResult<()> { bail!("message") })().unwrap_err();
        assert_eq!(&format!("{err}"), "message");
    }

    #[gtest]
    fn test_macro_ensure() {
        let err = (|| {
            ensure!(false, "message");
            Ok(())
        })()
        .unwrap_err();
        assert_eq!(&format!("{err}"), "message");
    }
}
