// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub use crate as inline_cpp_generated_bindings;
use inline_cpp_macro::inline_cpp;

macro_rules! global_cpp {
    ($($t:tt)*) => {};
}

global_cpp! {
    #include <cstddef>
    #include "rs_bindings_from_cc/test/bazel/rule/cpp_add.h"

    inline size_t CppAdd(size_t a, size_t b) { return Add(a, b); }
}

pub fn add(a: usize, b: usize) -> usize {
    crate::CppAdd(a, b)
}

pub fn multiply(a: usize, b: usize) -> usize {
    let mul = inline_cpp! {
        (size_t a, size_t b) -> size_t {
            return a * b;
        }
    };
    mul(a, b)
}
