# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "golden_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "special_linking",
    hdrs = ["special_linking.h"],
    aspect_hints = ["//features:experimental"],
)

golden_test(
    name = "special_linking_golden_test",
    basename = "special_linking",
    cc_library = "special_linking",
    golden_cc = "special_linking_api_impl.cc",
    golden_rs = "special_linking_rs_api.rs",
)

crubit_rust_test(
    name = "special_linking_test",
    srcs = ["test.rs"],
    cc_deps = [":special_linking"],
)
