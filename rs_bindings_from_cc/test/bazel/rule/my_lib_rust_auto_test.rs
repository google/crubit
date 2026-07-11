// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

fn main() {
    if my_lib_rust_auto_intermediate_cc_lib::CppAdd(40, 2) != 42 {
        std::process::exit(1);
    }
    std::process::exit(0);
}
