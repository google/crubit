# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs:cc_public_library.bzl", "cc_internal_library")

package(
    default_testonly = 1,
    default_visibility = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/G_cc_public_library:__pkg__",
    ],
)

cc_internal_library(
    name = "a",
    hdrs = ["a.h"],
    aspect_hints = ["//features:supported"],
    implements = ["//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/G_cc_public_library:a"],
)
