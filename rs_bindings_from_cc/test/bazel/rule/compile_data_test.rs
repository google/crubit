// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::expect_that;
use googletest::gtest;
use googletest::matchers::eq;

#[gtest]
fn test_source_compile_data() {
    expect_that!(
        compile_data_lib::compile_data_ns::SOURCE_DATA.trim_end(),
        eq("Hello from compile_data!")
    );
}

#[gtest]
fn test_generated_compile_data() {
    expect_that!(
        compile_data_lib::compile_data_ns::GENERATED_DATA.trim_end(),
        eq("Hello from generated compile_data!")
    );
}

#[gtest]
fn test_answer() {
    expect_that!(compile_data_lib::compile_data_ns::answer_from_rust(), eq(42));
}
