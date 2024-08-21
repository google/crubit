// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_bridge_type_as_function_arg() {
    let x = bridging_lib::MyRustStruct::new("hello");
    assert_eq!(bridging_lib::CalStructSize(x), 5);
}

#[gtest]
fn test_bridge_type_as_return_value() {
    assert_eq!(bridging_lib::ReturnHelloWorldStruct().s, String::from("hello world"));
}

#[gtest]
fn test_bridge_type_two_args_are_bridged() {
    let x = bridging_lib::MyRustStruct::new("hello");
    let y = bridging_lib::MyRustStruct::new(" world");
    assert_eq!(bridging_lib::Concat(x, y).s, String::from("hello world"));
}

#[gtest]
fn test_round_trip() {
    let x = bridging_lib::MyRustStruct::new("hello");
    let y = bridging_lib::PadADot(x);
    assert_eq!(y.s, "hello.");
}
