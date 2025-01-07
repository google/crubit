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

    /// Consolidates any errors into a single error message.
    ////
    /// Returns `Ok` if no errors have been added.
    pub fn consolidate(&self) -> Result<(), Error> {
        let errors = self.list.take();
        if !errors.is_empty() {
            return Err(Error::from(error_report::ErrorList::from(errors)));
        }
        Ok(())
    }

    /// If `res` is an error, adds the error to the list and consolidates the
    /// list into a single error message.
    ///
    /// Returns `res` unchanged if it is `Ok`.
    pub fn consolidate_on_err<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
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
