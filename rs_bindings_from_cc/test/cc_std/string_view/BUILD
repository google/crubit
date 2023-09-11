# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "string_view_apis",
    hdrs = ["string_view_apis.h"],
)

crubit_rust_test(
    name = "string_view",
    srcs = ["test.rs"],
    cc_deps = [
        ":string_view_apis",
        "//support/cc_std",
    ],
)
