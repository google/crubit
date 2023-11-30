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
