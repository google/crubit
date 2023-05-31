// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct LeafRsType {
    field: u8,
}

pub fn wrap(x: u8) -> LeafRsType {
    LeafRsType { field: x }
}

pub fn unwrap(x: LeafRsType) -> u8 {
    x.field
}
