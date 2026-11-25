// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate cpp_std;

pub fn explicit_cpp_std_works() -> bool {
    let _: Option<cpp_std::string_view> = None;
    true
}
