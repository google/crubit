// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::cpp_enum;
use open_enum::open_enum;

#[open_enum(allow_alias)]
#[cpp_enum(kind = "enum class")]
#[repr(i32)]
#[derive(Copy, Clone, Default)]
pub enum Color {
    Red,
    Blue,
    Green = 5,
    Gray = 5,
    Magenta = 7,
}
