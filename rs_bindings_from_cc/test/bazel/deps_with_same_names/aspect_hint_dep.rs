// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn f() -> i32 {
    lib_aspect_hint_dep::f_aspect_hint_dep() + lib_aspect_hint_dep::hello_from_extra()
}
