// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Ensures that basic usages of `crubit_annotate` compile successfully.
//!
//! Note that this check does not actually test the functionality of the annotations
//! as-consumed by Crubit, only that usages of the procedural macros compile successfully.

#![allow(unused)]

#[crubit_annotate::cpp_layout_equivalent(cpp_type = "some_str", include_path = "some/path.h")]
struct LayoutEquivalent {}

#[crubit_annotate::cpp_convertible(
    cpp_type = "some_str",
    include_path = "some/path.h",
    cpp_to_rust_converter = "::some_str_to_rust",
    rust_to_cpp_converter = "::rust_to_some_str"
)]
struct Convertible {}

#[crubit_annotate::cpp_name("CppName")]
struct RustName {}

impl RustName {
    #[crubit_annotate::cpp_name("create")]
    fn new() -> RustName {
        Self {}
    }
}

#[crubit_annotate::cpp_enum(kind = "enum")]
#[repr(transparent)]
struct Enum(i32);

#[crubit_annotate::cpp_enum(kind = "enum class")]
#[repr(transparent)]
struct EnumClass(i32);
