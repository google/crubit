# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
    "crubit_rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "cc_helper_functions",
    testonly = 1,
    srcs = ["cc_helper_functions.cc"],
    hdrs = ["cc_helper_functions.h"],
)

rust_api_from_cpp(
    name = "cc_helper_functions_rust",
    testonly = 1,
    cpp_target = ":cc_helper_functions",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":cc_helper_functions_rust",
        "//support:cpp_std",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "rust_variant_lib",
    srcs = ["rust_variant_lib.rs"],
    edition = "2024",
    deps = [
        "//support:cpp_std",
        "//support:ctor",
    ],
)

cpp_api_from_rust(
    name = "rust_variant_lib_cc_api",
    testonly = 1,
    crate = ":rust_variant_lib",
)

crubit_cc_test(
    name = "cpp_api_test",
    srcs = ["cpp_api_test.cc"],
    # TODO(b/575815022): Re-enable once `cc_std` links on macOS.
    tags = [
        "no_ios",
        "no_macos",
        "no_test_android_x86",
    ],
    deps = [
        ":rust_variant_lib_cc_api",
        "//support/rs_std:char",
        "@googletest//:gtest_main",
    ],
)
