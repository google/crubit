# Part of the Crubit project, under the Apache License v2.0 with LLVM.
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")

package(
    default_testonly = 1,
)

# TODO(b/541216309): Remove when CL 983136933 reaches stable crosstool.
_tags = [
    "notap",
    "noguitar",
    "manual",
    "nobuilder",
]

cc_library(
    name = "key_lib",
    hdrs = ["key_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    tags = _tags,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/B_cc_library:a",
        "//support:annotations",
    ],
)

cc_library(
    name = "client_cc",
    hdrs = ["client.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    deps = [
        ":key_lib",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/B_cc_library:a",
    ],
)

rust_api_from_cpp(
    name = "client_cc_rust",
    cpp_target = ":client_cc",
    tags = _tags,
)

rust_library(
    name = "client_rust",
    srcs = ["client.rs"],
    edition = "2024",
    tags = _tags,
    deps = [":client_cc_rust"],
)

build_test(
    name = "build_test",
    tags = _tags,
    targets = [
        ":client_rust",
    ],
)
