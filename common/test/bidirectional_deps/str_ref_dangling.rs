// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Must not compile: the result of `HoldStr` borrows from `s`, which is
//! dropped before the result is used.

pub fn dangling() -> u8 {
    let h = {
        let s = String::from("hello");
        str_ref_cc_lib::crubit::HoldStr(&s)
    };
    str_ref_cc_lib::crubit::FirstByte(h)
}
