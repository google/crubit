// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn read_derived_transitively(w: &rust_api::DerivedWrapper) -> i32 {
    rust_api::read_derived(w) + 1
}
