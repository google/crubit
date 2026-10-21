# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "test_helpers_rust",
    testonly = 1,
    cpp_target = ":test_helpers",
)

crubit_rust_test(
    name = "shared_ptr_test",
    srcs = ["shared_ptr_test.rs"],
    deps = [
        ":test_helpers_rust",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)
