// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn take_msg_a(a: a_extension_rust_proto::MsgA) -> i32 {
    a.x()
}

pub fn take_msg_b(b: b_extension_rust_proto::MsgB) -> i32 {
    b.y()
}
