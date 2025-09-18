# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "cc_helper_functions",
    hdrs = ["cc_helper_functions.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":cc_helper_functions",
        "//support/public:cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)
