// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `crate_name_test.cc`.  The main
//! focus is testing bindings of API that depends on types exported by another
//! crate - functions in the `test_api.rs` crate use types exported by
//! `other_crate.rs`.

pub fn create_struct(i: i32) -> other_crate::SomeStruct {
    other_crate::SomeStruct(i)
}

pub fn extract_int(s: other_crate::SomeStruct) -> i32 {
    s.0
}

// b/292231336: Crubit currently fails to generate bindings for this function.
// Error message: Type std::string::String comes from the `alloc` crate, but no
// `--crate-header` was specified for this crate.
pub fn return_a_type_from_a_rust_toolchain_crate() -> String {
    "String".to_owned()
}
