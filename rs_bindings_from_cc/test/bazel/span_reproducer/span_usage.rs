// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn f() -> i32 {
    // We don't even need to call anything, just importing the dependency is enough.
    // If the bindings fail to compile, this library will fail to build.
    0
}
