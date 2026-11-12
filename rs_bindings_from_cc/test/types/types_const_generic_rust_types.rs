// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RustTaggedArray<T, const N: usize, const TAG: i32>(pub [T; N]);
