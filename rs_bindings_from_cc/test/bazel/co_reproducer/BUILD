# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Regression test for c9::Co crate name hardcoding.
#
# Historically, the Crubit generator hardcoded the name of the `c9::Co`
# bindings crate as `::co`. This caused issues when:
# 1. Crate renaming was enabled (which mangles the crate name to its label-encoded version).
# 2. A user-defined Rust library also named `co` was present in the dependency
#    graph (e.g. via `additional_rust_srcs`), causing `::co` to resolve to the
#    wrong crate.
#
# This test verifies that we can compile bindings that use `c9::Co` even when
# a conflicting user-defined `co` crate is present in the dependencies,
# provided that crate renaming is enabled.

load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

package(
    default_testonly = 1,
)

rust_library(
    name = "co",
    srcs = ["rust_co.rs"],
)

additional_rust_srcs_for_crubit_bindings(
    name = "extra_srcs",
    srcs = ["extra.rs"],
    deps = [":co"],
)

cc_library(
    name = "co_usage_cc",
    hdrs = ["co_usage.h"],
    aspect_hints = [
        ":extra_srcs",
        "//features:supported",
    ],
    deps = [
        "//util/c9:co",
    ],
)

rust_library(
    name = "co_usage_rust",
    srcs = ["co_usage.rs"],
    cc_deps = [":co_usage_cc"],
)

build_test(
    name = "build_test",
    targets = [
        ":co_usage_rust",
    ],
)
