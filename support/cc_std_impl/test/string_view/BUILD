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
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "string_view_rs_apis",
    srcs = ["string_view_rs_apis.rs"],
    edition = "2024",
    deps = ["//support/cc_std"],
)

cpp_api_from_rust(
    name = "string_view_rs_apis_cc",
    testonly = 1,
    crate = ":string_view_rs_apis",
)

crubit_test_cc_library(
    name = "string_view_cc_apis",
    hdrs = ["string_view_cc_apis.h"],
)

rust_api_from_cpp(
    name = "string_view_cc_apis_rust",
    cpp_target = ":string_view_cc_apis",
)

crubit_rust_test(
    name = "string_view_rs_test",
    srcs = ["test.rs"],
    deps = [
        ":string_view_cc_apis_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

crubit_cc_test(
    name = "string_view_cc_test",
    srcs = ["test.cc"],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    # TODO(b/575815022): Re-enable once `cc_std` links on macOS.
    tags = [
        "no_macos",
        "no_windows",
    ],
    deps = [
        ":string_view_rs_apis_cc",
        "@googletest//:gtest_main",
    ],
)
