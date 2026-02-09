# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

crubit_rust_test(
    name = "unique_ptr_test",
    srcs = ["unique_ptr_test.rs"],
    cc_deps = [
        ":test_helpers",
        "//support/public:cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "unique_ptr_dyn_test_helpers",
    testonly = 1,
    srcs = ["unique_ptr_dyn_test_helpers.cc"],
    hdrs = ["unique_ptr_dyn_test_helpers.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

crubit_rust_test(
    name = "unique_ptr_dyn_test",
    srcs = ["unique_ptr_dyn_test.rs"],
    cc_deps = [
        ":unique_ptr_dyn_test_helpers",
        "//support/public:cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
