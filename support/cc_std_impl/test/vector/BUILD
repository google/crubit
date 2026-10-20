# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "cc_helper_functions",
    testonly = True,
    hdrs = ["cc_helper_functions.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":cc_movable_types",
        "@abseil-cpp//absl/base:no_destructor",
    ],
)

rust_api_from_cpp(
    name = "cc_helper_functions_rust",
    testonly = True,
    cpp_target = ":cc_helper_functions",
)

cc_library(
    name = "cc_movable_types",
    testonly = True,
    hdrs = ["cc_movable_types.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "cc_movable_types_rust",
    testonly = True,
    cpp_target = ":cc_movable_types",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":cc_helper_functions_rust",
        ":cc_movable_types_rust",
        "//support/cc_std",
        "//support/ffi_11",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
