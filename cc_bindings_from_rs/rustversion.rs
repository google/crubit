// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A date-based stable/beta-aware rustversion::since and rustversion::before.
//!
//! To only include a piece of code on versions of rustc after or on `YYYY-MM-DD`, use [`since`]:
//!
//! ```
//! #[rustversion::since(YYYY-MM-DD)]
//! my_code();
//! ```
//!
//! This will include `my_code()` on the `YYYY-MM-DD` nightly, all subsequent nightlies, and all
//! stable releases branched from that date or later.
//!
//! To only include a piece of code on versions of rustc before that date, use [`before`]:
//!
//! ```
//! #[rustversion::before(YYYY-MM-DD)]
//! my_code_that_only_works_on_old_rustc();
//! ```
//!
//! ## Why is there no `rustversion::nightly`?
//!
//! Code doesn't work on _all_ nightlies, it works on nightlies after a certain date. Typically,
//! people use `rustversion::nightly` to mean "recentish nightlies that include changes after the
//! current stable release". This will break on the next stable release, which does include those
//! changes. If you use a more specific date (even if it is not exactly correct), it will work for
//! the recent nightlies, as well as the next stable release, and all stable releases after that.

pub use rustversion_impl::before;
pub use rustversion_impl::since;

#[doc(hidden)]
pub mod macro_internal {
    pub use ::rustversion::any;
}
