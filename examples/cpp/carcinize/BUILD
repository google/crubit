# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//support/extract_cpp_from_rust:rust_library_with_embedded_cpp.bzl",
    "rust_library_with_embedded_cpp",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

# Baseline C++ geometry library before migration.
cc_library(
    name = "point_before",
    srcs = ["point.cc"],
    hdrs = ["point.h"],
    aspect_hints = ["//features:supported"],
)

# Migrated geometry library using embedded C++.
rust_library_with_embedded_cpp(
    name = "point_rs",
    srcs = ["point_rs.rs"],
    deps = [
        "//support:ctor",
        "//support/ffi_11",
    ],
)

# Re-exports :point_rs to C++ callers under the original target name.
cc_bindings_from_rust(
    name = "point",
    crate = ":point_rs",
)

# Idiomatic pure Rust implementation after refactoring.
rust_library(
    name = "point_refactored",
    srcs = ["point_refactored.rs"],
)

# Baseline C++ math utilities library before incomplete migration.
cc_library(
    name = "math_utils_before",
    hdrs = ["math_utils.h"],
    aspect_hints = ["//features:supported"],
)

# Migrated math utilities library with fallback global_cpp! block.
rust_library_with_embedded_cpp(
    name = "math_utils_rs",
    srcs = ["math_utils_rs.rs"],
    deps = [
        "//support:ctor",
        "//support/ffi_11",
    ],
)

# Re-exports :math_utils_rs to C++ callers.
cc_bindings_from_rust(
    name = "math_utils",
    crate = ":math_utils_rs",
)

rust_test(
    name = "point_test",
    srcs = ["point_test.rs"],
    harness = "libtest",
    deps = [
        ":point_refactored",
        ":point_rs",
        "@crate_index//:googletest",
    ],
)

rust_test(
    name = "math_utils_test",
    srcs = ["math_utils_test.rs"],
    harness = "libtest",
    deps = [
        ":math_utils_rs",
        "@crate_index//:googletest",
    ],
)

rust_binary(
    name = "main_rs",
    srcs = ["main.rs"],
    deps = [
        ":math_utils_rs",
        ":point_refactored",
        ":point_rs",
    ],
)

cc_binary(
    name = "main_cc",
    srcs = ["main.cc"],
    deps = [
        ":math_utils",
        ":point",
    ],
)
