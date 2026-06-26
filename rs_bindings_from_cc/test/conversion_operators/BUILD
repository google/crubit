# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "foreign_type",
    hdrs = ["foreign_type.h"],
    aspect_hints = ["//features:supported"],
)

crubit_test_cc_library(
    name = "conversion_operators",
    srcs = ["conversion_operators.cc"],
    hdrs = ["conversion_operators.h"],
    aspect_hints = ["//features:assume_this_lifetimes"],
    deps = [":foreign_type"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":conversion_operators"],
    deps = [
        "//support:cref",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)
