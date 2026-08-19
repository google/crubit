// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::{global_cpp, inline_cpp};

global_cpp! {
    #include "support/rs_std/str_ref.h"
    #include "third_party/absl/strings/str_cat.h"
}

/// Formats a greeting using C++ `absl::StrCat`.
pub fn format_greeting(name: &str) -> String {
    let greet = inline_cpp! {
        (rs_std::StrRef name) -> std::string {
            return absl::StrCat("Hello, ", name);
        }
    };
    let cpp_str = greet(name);
    cpp_str.to_string().expect("Valid UTF-8")
}
