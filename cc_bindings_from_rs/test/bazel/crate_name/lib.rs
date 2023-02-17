// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `crate_name_test.cc`.
//!
//! The file name (i.e. `lib.rs`) is typical for Cargo crates.  The crate
//! is given a real name via Bazel BUILD file:
//!
//! ```
//! rust_library(
//!    name = "custom_crate_name",
//!    srcs = ["lib.rs"],
//! )
//! ```
//!
//! This test verifies that `cc_bindings_from_rs` is invoked with
//! `--crate_name=custom_crate_name` - without the cmdline argument
//! `cc_bindings_from_rs` would think that the crate name is `lib`.
//! - `lib::get_the_answer()` wouldn't compile if used in the generated
//!   `...cc_api_impl.rs`.
//! - `namespace lib` is also undesirable in the generated `...cc_api.h` (this
//!   is slightly less important because in the long-term the C++ namespace
//!   might not depend on the crate name - see
//!   <internal link>).

pub fn get_the_answer() -> i32 {
    42
}
