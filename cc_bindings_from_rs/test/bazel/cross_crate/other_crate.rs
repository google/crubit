// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a dependency of `test_api.rs` - types exported by
//! `other_crate.rs` are used in public API exposed by `test_api.rs`.

pub struct SomeStruct(pub i32);

#[crubit_annotate::cpp_layout_equivalent(
    cpp_type = "crubit::test::MyStatusOr<{T}>",
    include_path = "cc_bindings_from_rs/test/bazel/cross_crate/other_crate_types.h"
)]
#[repr(C)]
pub struct MyStatusOr<T> {
    pub has_value: bool,
    pub value: T,
}

#[crubit_annotate::cpp_specialization(
    cpp_type = "crubit::test::MyStatus",
    include_path = "cc_bindings_from_rs/test/bazel/cross_crate/other_crate_types.h"
)]
pub type MyStatus = MyStatusOr<()>;
