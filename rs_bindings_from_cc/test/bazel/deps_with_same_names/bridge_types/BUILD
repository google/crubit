# Part of the Crubit project, under the Apache License v2.0 with LLVM.
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)

package(
    default_testonly = 1,
)

_compatible_with_flag = select({
    "//rs_bindings_from_cc/test/bazel/deps_with_same_names:use_label_encoded_names_for_deps_enabled": [],
    "//conditions:default": ["//third_party/bazel_platforms:incompatible"],
})

_tags = [
    "notap",
    "noguitar",
    "manual",
    "nobuilder",
]

# Test case 1: absl::StatusOr user
cc_library(
    name = "status_user_cc",
    hdrs = ["status_user.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "@abseil-cpp//absl/status:statusor",
    ],
)

rust_library(
    name = "status_user_rust",
    srcs = ["status_user.rs"],
    cc_deps = [":status_user_cc"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# Test case 2: Same-name deps with bridging
cc_library(
    name = "key_lib",
    hdrs = ["key_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/B_cc_library:a",
        "//support:annotations",
    ],
)

cc_library(
    name = "client_cc",
    hdrs = ["client.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        ":key_lib",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/B_cc_library:a",
    ],
)

rust_library(
    name = "client_rust",
    srcs = ["client.rs"],
    cc_deps = [":client_cc"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

rust_test(
    name = "status_user_test",
    srcs = ["status_user_test.rs"],
    harness = "libtest",
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        ":status_user_rust",
        "@crate_index//:googletest",
    ],
)

build_test(
    name = "build_test",
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    targets = [
        ":status_user_rust",
        ":status_user_test",
        ":client_rust",
    ],
)
