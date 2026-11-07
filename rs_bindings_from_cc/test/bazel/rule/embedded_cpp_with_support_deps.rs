// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use inline_cpp_macro::{global_cpp, inline_cpp};

global_cpp! {
    #include "support/rs_std/str_ref.h"
}

pub fn get_hello_size(s: &str) -> usize {
    let get_size = inline_cpp! {
        (rs::StrRef name) -> size_t {
          return name.size();
        }
    };
    get_size(s)
}
