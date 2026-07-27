# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs:cc_public_library.bzl", "cc_internal_library")

package(
    default_testonly = 1,
    default_visibility = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/H_cc_public_library:__pkg__",
    ],
)

_tags = [
    "notap",
    "noguitar",
    "manual",
    "nobuilder",
]

cc_internal_library(
    name = "a",
    hdrs = ["a.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = select({
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names:use_label_encoded_names_for_deps_enabled": [],
        "//conditions:default": ["//third_party/bazel_platforms:incompatible"],
    }),
    implements = ["//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/H_cc_public_library:a"],
)
