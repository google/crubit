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

rust_library(
    name = "shared_ptr",
    testonly = 1,
    srcs = [
        "shared_ptr.rs",
    ],
    proc_macro_deps = ["//support:crubit_annotate"],
    deps = ["//support/cc_std"],
)

golden_test(
    name = "shared_ptr_golden_test",
    basename = "shared_ptr",
    golden_h = "shared_ptr_cc_api.h",
    golden_rs = "shared_ptr_cc_api_impl.rs",
    rust_library = "shared_ptr",
)

cc_bindings_from_rust(
    name = "shared_ptr_cc_api",
    testonly = 1,
    crate = ":shared_ptr",
)

crubit_cc_test(
    name = "shared_ptr_test",
    srcs = ["shared_ptr_test.cc"],
    deps = [
        ":shared_ptr_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
