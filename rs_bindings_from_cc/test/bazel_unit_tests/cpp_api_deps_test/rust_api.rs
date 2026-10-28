// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub use base_cc::DerivedWrapper;

pub fn read_derived(w: &DerivedWrapper) -> i32 {
    w.value
}
