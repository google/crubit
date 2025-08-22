// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Placeholder module for working with collections of Errors that have not yet
//! been issued.
//!
//! This approach is intended to allow for continuing with compilation after the
//! first error has been encountered, allowing multiple errors to be issued and
//! allowing the bindings generator to guess at the resulting API. This last
//! step is especially import in that it allows us to e.g. generate a mock
//! implementation for a function that is declared in a header, even if the
//! *real* function can't be generated. We can then issue a descriptive error
//! from the Rust compiler when the user attempts to invoke the mock function.

use arc_anyhow::Error;
use core::cell::RefCell;

// Re-exported for macro use.
#[doc(hidden)]
pub mod macro_internals {
    pub use error_report::anyhow;
}

/// A collection of errors that have not yet been issued.
///
/// A non-empty collection of errors must either be `consolidate`d or
/// `discard`ed before it is dropped. Failure to do so will result in a panic.
#[derive(Default)]
pub struct Errors {
    list: RefCell<Vec<Error>>,
}

impl Errors {
    pub fn new() -> Errors {
        Default::default()
    }

    pub fn add(&self, error: Error) {
        self.list.borrow_mut().push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.list.borrow().is_empty()
    }

    /// If the supplied result contains an error, add it to the list and return None. Otherwise
    /// return the result.
    pub fn consume_error<T>(&self, r: Result<T, Error>) -> Option<T> {
        match r {
            Ok(t) => Some(t),
            Err(e) => {
                self.add(e);
                None
            }
        }
    }

    /// Consolidates any errors into a single error message.
    ////
    /// Returns `Ok` if no errors have been added.
    pub fn consolidate(&self) -> Result<(), error_report::ErrorList> {
        let errors = self.list.take();
        if !errors.is_empty() {
            return Err(error_report::ErrorList::from(errors));
        }
        Ok(())
    }

    /// If `res` is an error, adds the error to the list and consolidates the
    /// list into a single error message.
    ///
    /// Returns `res` unchanged if it is `Ok`.
    pub fn consolidate_on_err<T>(
        &self,
        res: Result<T, Error>,
    ) -> Result<T, error_report::ErrorList> {
        res.map_err(|error| {
            self.add(error);
            self.consolidate().unwrap_err()
        })
    }

    /// Discards without reporting errors.
    pub fn discard(&self) {
        self.list.take();
    }
}

impl Drop for Errors {
    fn drop(&mut self) {
        let errors = self.list.get_mut();
        if !errors.is_empty() {
            panic!(
                "`Error`s dropped without first calling `consolidate` or `discard`:\n{}",
                errors.iter().map(|error| error.to_string()).collect::<Vec<_>>().join("\n")
            );
        }
    }
}

/// Similar to `bail!`, but accepts an `Errors` instance as the first argument.
///
/// Only use this macro if it is not possible to continue after an error has
/// been generated. Prefer instead to add an entry to `Errors`.
///
/// The error will be added to the provided `Errors` list before returning
/// `Err(WillError)`.
///
/// This macro is intended to be used in a function that returns `ErrorsOr<T>`.
#[macro_export]
macro_rules! bail_to_errors {
    ($errors:expr, $($arg:tt)*) => {
        {
        $errors.add(anyhow!($($arg)*));
        return Err($crate::WillError)
        }
    }
}

/// A promise that an error has been added to an `Errors`.
pub struct WillError;

/// A `Result` alias indicating that the value may not be available due to
/// errors that have been added to an `Errors` instance.
///
/// Only use this macro if it is not possible to continue after an error has
/// been generated. Prefer instead to add an entry to `Errors` and continue
/// successfully.
pub type ErrorsOr<T> = Result<T, WillError>;
