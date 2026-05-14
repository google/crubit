// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use my_proc_macro::make_answer;

make_answer!();

pub fn do_something() -> i32 {
    answer()
}
