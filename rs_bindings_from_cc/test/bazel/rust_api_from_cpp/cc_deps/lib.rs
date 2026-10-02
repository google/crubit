// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn call_dep_double(x: i32) -> i32 {
    // dep is the crate name for the dep target.
    dep::double_value(x)
}
