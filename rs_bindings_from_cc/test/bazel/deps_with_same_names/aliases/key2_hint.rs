// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate std;

pub fn key2_func() -> i32 {
    foo::rust_foo_func() + aliased_foo::cc_foo_func() + aliased_bar::cc_bar_func()
}
