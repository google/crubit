# Part of the Crubit project, under the Apache License v2.0 with LLVM.
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

package(
    default_testonly = 1,
)

# Test case 2: Same-name deps with bridging
cc_library(
    name = "key_lib",
    hdrs = ["key_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
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
)

build_test(
    name = "build_test",
    targets = [
        ":client_rust",
    ],
)
