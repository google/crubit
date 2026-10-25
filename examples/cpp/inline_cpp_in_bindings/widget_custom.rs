// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use inline_cpp_macro::{global_cpp, inline_cpp};

global_cpp! {
    #include "examples/cpp/inline_cpp_in_bindings/widget.h"
}

/// Calls the C++ function template `widget::Twice<int>`.
pub fn twice_int(x: i32) -> i32 {
    let f = inline_cpp! {
        (int x) -> int {
          return widget::Twice<int>(x);
        }
    };
    f(x)
}
