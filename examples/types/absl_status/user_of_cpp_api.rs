// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::{eq, ok as is_ok, status_is};
use googletest::{expect_that, gtest};
use status::StatusCodeError;

#[gtest]
fn test_returns_status() {
    let status = cpp_api::ReturnsStatus(true);
    expect_that!(status, is_ok(eq(&())));

    let status = cpp_api::ReturnsStatus(false);
    expect_that!(
        status,
        status_is(StatusCodeError::INTERNAL).with_message(eq("Something went wrong, oh no!"))
    );
}

#[gtest]
fn test_returns_status_or_int() {
    let status_or_int = cpp_api::ReturnsStatusOrInt(true);
    expect_that!(status_or_int, is_ok(eq(&42)));

    let status_or_int = cpp_api::ReturnsStatusOrInt(false);
    expect_that!(
        status_or_int,
        status_is(StatusCodeError::INTERNAL).with_message(eq("Something went wrong, oh no!"))
    );
}

#[gtest]
fn test_returns_status_with_proto() {
    let mut msg = example_rust_proto::ExampleProto::new();
    let status = cpp_api::ReturnsStatusWithProto(true, msg.as_mut());
    expect_that!(status, is_ok(eq(&())));
    expect_that!(msg.value(), eq(42));

    let mut msg = example_rust_proto::ExampleProto::new();
    let status = cpp_api::ReturnsStatusWithProto(false, msg.as_mut());
    expect_that!(
        status,
        status_is(StatusCodeError::INTERNAL).with_message(eq("Something went wrong, oh no!"))
    );
}
