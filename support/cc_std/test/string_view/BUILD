# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "string_view_rs_apis",
    srcs = ["string_view_rs_apis.rs"],
    cc_deps = ["//support/cc_std"],
)

cc_bindings_from_rust(
    name = "string_view_rs_apis_cc",
    testonly = 1,
    crate = ":string_view_rs_apis",
)

crubit_test_cc_library(
    name = "string_view_cc_apis",
    hdrs = ["string_view_cc_apis.h"],
)

crubit_rust_test(
    name = "string_view_rs_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":string_view_cc_apis",
        "//support/public:cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_cc_test(
    name = "string_view_cc_test",
    srcs = ["test.cc"],
    deps = [
        ":string_view_rs_apis_cc",
        "@googletest//:gtest_main",
    ],
)
