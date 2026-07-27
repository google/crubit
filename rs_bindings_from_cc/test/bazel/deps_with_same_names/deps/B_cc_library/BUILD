# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_library.bzl", "cc_library")

package(
    default_testonly = 1,
    default_visibility = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names:__subpackages__",
    ],
)

_tags = [
    "notap",
    "noguitar",
    "manual",
    "nobuilder",
]

cc_library(
    name = "a",
    hdrs = ["a.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
)
