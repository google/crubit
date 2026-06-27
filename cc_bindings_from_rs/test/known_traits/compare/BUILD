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
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "compare",
    testonly = 1,
    srcs = ["compare.rs"],
)

cc_bindings_from_rust(
    name = "compare_cc_api",
    testonly = 1,
    crate = ":compare",
)

golden_test(
    name = "compare_cc_api_golden",
    basename = "compare",
    golden_h = "compare_cc_api.h",
    golden_rs = "compare_cc_api_impl.rs",
    rust_library = "compare",
)

crubit_cc_test(
    name = "compare_test",
    srcs = ["compare_test.cc"],
    deps = [
        ":compare_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
