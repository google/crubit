// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[doc = "CRUBIT_ANNOTATE: cpp_enum=enum class"]
#[repr(transparent)]
pub struct SomeEnum(i32);
impl SomeEnum {
    pub const VARIANT_0: SomeEnum = SomeEnum(0);
    pub const VARIANT_1: SomeEnum = SomeEnum(1);
    pub const VARIANT_2: SomeEnum = SomeEnum(2);
}

pub struct SomeStruct {
    pub f: i32,
}
