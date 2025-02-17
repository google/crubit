// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_proto_api_is_wrapped() {
    let my_msg = my_rust_proto::MyMessage::new();
    my_proto_api::SetStringOnMyMessage(&my_msg, 123);
    expect_that!(my_msg.my_num(), eq(123));
}
