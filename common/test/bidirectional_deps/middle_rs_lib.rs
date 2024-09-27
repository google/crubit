// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use leaf_cc_lib::crubit::*;

pub fn wrap(x: u8) -> LeafCcType {
    Wrap(x)
}
pub fn unwrap(x: LeafCcType) -> u8 {
    Unwrap(x)
}

pub fn wrap_enum(x: u8) -> LeafCcEnum {
    WrapEnum(x)
}
pub fn unwrap_enum(x: LeafCcEnum) -> u8 {
    UnwrapEnum(x)
}

pub use leaf_cc_lib::crubit::LeafCcTypeAlias as LeafCcTypeAlias2;
pub type LeafCcTypeAlias3 = LeafCcTypeAlias;
