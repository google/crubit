# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")

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

proto_library(
    name = "proto",
    srcs = ["proto.proto"],
)

cc_proto_library(
    name = "a_cc_proto",
    tags = _tags,
    target_compatible_with = select({
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names:use_label_encoded_names_for_deps_enabled": [],
        "//conditions:default": ["//third_party/bazel_platforms:incompatible"],
    }),
    deps = [":proto"],
)
