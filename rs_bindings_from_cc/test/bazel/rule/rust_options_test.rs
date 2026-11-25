// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::expect_that;
use googletest::gtest;
use googletest::matchers::eq;

#[gtest]
fn test_crate_features() {
    expect_that!(rust_options_lib::bonus_value(), eq(42));
    expect_that!(rust_options_lib::disabled_feature_enabled(), eq(false));
}

#[gtest]
fn test_rustc_flags() {
    expect_that!(rust_options_lib::custom_flags_enabled(), eq(true));
}

#[gtest]
fn test_rustc_env_and_env_files() {
    expect_that!(rust_options_lib::FROM_RUSTC_ENV, eq("hello_from_rustc_env"));
    expect_that!(rust_options_lib::FROM_ENV_FILE, eq("hello_from_env_file"));
    expect_that!(rust_options_lib::FROM_LOCATION_ENV.trim_end(), eq("Hello from compile_data!"));
}
