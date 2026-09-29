# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

package(
    default_testonly = 1,
    default_visibility = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names:__subpackages__",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "extra_srcs",
    srcs = ["extra.rs"],
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/rust_library_a:a",
    ],
)

cc_library(
    name = "a",
    hdrs = ["a.h"],
    aspect_hints = [
        ":extra_srcs",
        "//features:supported",
    ],
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/B_cc_library:a",
    ],
)
