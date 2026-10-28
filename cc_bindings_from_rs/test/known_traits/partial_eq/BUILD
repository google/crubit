# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "partial_eq",
    testonly = 1,
    srcs = ["partial_eq.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "partial_eq_cc_api",
    testonly = 1,
    crate = ":partial_eq",
)

golden_test(
    name = "partial_eq_cc_api_golden",
    basename = "partial_eq",
    golden_h = "partial_eq_cc_api.h",
    golden_rs = "partial_eq_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "partial_eq",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "partial_eq_test",
    srcs = ["partial_eq_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":partial_eq_cc_api",
        "//support/rs_std:str_ref",
        "@googletest//:gtest_main",
    ],
)
