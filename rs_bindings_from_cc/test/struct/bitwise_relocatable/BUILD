# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "bitwise_relocatable",
    srcs = ["bitwise_relocatable.cc"],
    hdrs = ["bitwise_relocatable.h"],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "bitwise_relocatable_rust",
    cpp_target = ":bitwise_relocatable",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":bitwise_relocatable_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
