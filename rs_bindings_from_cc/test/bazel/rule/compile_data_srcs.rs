// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Contents of a checked-in file listed in `compile_data`.
pub const SOURCE_DATA: &str = include_str!("testdata/compile_data.yaml");

/// Contents of a generated file listed in `compile_data`.
pub const GENERATED_DATA: &str = include_str!("generated_compile_data.txt");

pub fn answer_from_rust() -> i32 {
    crate::compile_data_ns::Answer()
}
