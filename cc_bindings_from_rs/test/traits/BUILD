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
    name = "traits",
    srcs = ["traits.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "traits_golden_test",
    basename = "traits",
    golden_h = "traits_cc_api.h",
    golden_rs = "traits_cc_api_impl.rs",
    rust_library = "traits",
)

cc_bindings_from_rust(
    name = "traits_cc_api",
    testonly = 1,
    crate = ":traits",
)

crubit_cc_test(
    name = "traits_test",
    srcs = ["traits_test.cc"],
    deps = [
        ":traits_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
