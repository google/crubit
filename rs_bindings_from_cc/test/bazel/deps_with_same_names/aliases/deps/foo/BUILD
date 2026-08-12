# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_library")

package(
    default_applicable_licenses = ["//:license"],
    default_testonly = 1,
    default_visibility = ["//rs_bindings_from_cc/test/bazel/deps_with_same_names:__subpackages__"],
)

rust_library(
    name = "foo",
    srcs = ["foo.rs"],
)
