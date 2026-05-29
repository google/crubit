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
    name = "partial_eq",
    testonly = 1,
    srcs = ["partial_eq.rs"],
)

cc_bindings_from_rust(
    name = "partial_eq_cc_api",
    testonly = 1,
    crate = ":partial_eq",
)

golden_test(
    name = "partial_eq_cc_api_golden",
    basename = "partial_eq",
    golden_h = "partial_eq_cc_api.h",
    golden_rs = "partial_eq_cc_api_impl.rs",
    rust_library = "partial_eq",
)

crubit_cc_test(
    name = "partial_eq_test",
    srcs = ["partial_eq_test.cc"],
    deps = [
        ":partial_eq_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
