// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use inline_cpp_macro::{global_cpp, inline_cpp};

global_cpp! {
    #include <stdint.h>
    #include "rs_bindings_from_cc/test/bazel/rule/math_dependency.h"
}

pub fn add(a: u32, b: u32) -> u32 {
    let cpp_add = inline_cpp! {
        (uint32_t a, uint32_t b) -> uint32_t {
            return math_dependency::Add(a, b);
        }
    };
    cpp_add(a, b)
}

impl crate::math_dependency::MyStruct {
    pub fn do_something(&self) {}
}
