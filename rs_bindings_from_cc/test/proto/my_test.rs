// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use forward_declare::CppCast;
use googletest::prelude::*;

#[gtest]
fn test_return_value() {
    let msg = my_proto_api::test::ReturnValue();

    expect_that!(msg.my_num(), eq(123));
}

#[gtest]
fn test_extract_from_value() {
    let mut msg = my_rust_proto::MyMessage::new();
    msg.set_my_num(321);
    let num = my_proto_api::test::ExtractFromValue(msg);

    expect_that!(num, eq(321));
}

#[gtest]
fn test_extract_from_const_ptr_or_ref() {
    let mut msg = my_rust_proto::MyMessage::new();
    msg.set_my_num(432);

    unsafe {
        expect_that!(my_proto_api::test::ExtractFromConstPtr(msg.as_view().cpp_cast()), eq(432));
        expect_that!(my_proto_api::test::ExtractFromConstRef(msg.as_view().cpp_cast()), eq(432));
    }
}

#[gtest]
fn test_extract_from_mutable_ptr_or_ref() {
    let mut msg = my_rust_proto::MyMessage::new();
    msg.set_my_num(543);

    unsafe {
        expect_that!(my_proto_api::test::ExtractFromMutablePtr(msg.as_mut().cpp_cast()), eq(543));
        expect_that!(my_proto_api::test::ExtractFromMutableRef(msg.as_mut().cpp_cast()), eq(543));
    }
}
