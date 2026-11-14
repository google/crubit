// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SameCrateRustType {
    pub x: i32,
}

#[crubit_annotate::allow_unbindable_type]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TaggedValue<T, const TAG: i64> {
    pub value: T,
}
