# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# This package tests that `rust_api_from_cpp` correctly propagates `cpp_deps`
# to the additional Rust sources (`srcs`), allowing them to use the generated
# Rust bindings of those dependencies.

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

package(
    default_applicable_licenses = ["//:license"],
    default_testonly = 1,
)

licenses(["notice"])

cc_library(
    name = "dep",
    hdrs = ["dep.h"],
    aspect_hints = ["//features:supported"],
)

cc_library(
    name = "lib",
    hdrs = ["lib.h"],
    aspect_hints = [":lib_rust.hint"],
)

rust_api_from_cpp(
    name = "lib_rust",
    srcs = ["lib.rs"],
    cpp_deps = [":dep"],
    cpp_target = ":lib",
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":lib_rust",
        "@crate_index//:googletest",
    ],
)
