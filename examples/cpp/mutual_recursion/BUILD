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
load("//bazel:cpp_api_from_rust.bzl", "cpp_api_from_rust")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")

licenses(["notice"])

# The original C++ library, which C++ callers keep depending on. `IsEven` is
# still implemented in C++, and `IsOdd` now forwards to Rust (`:is_odd`).
cc_library(
    name = "parity",
    srcs = ["parity.cc"],
    hdrs = ["parity.h"],
    deps = [
        ":is_odd_cc",
        ":parity_decls",
    ],

    # `:is_odd` calls `parity::IsEven`, which is defined in `parity.cc`. On the
    # link line, `:parity` comes before `:is_odd` (Bazel puts libraries before
    # their dependencies), so if nothing has pulled `parity.cc` into the link
    # by the time the linker reaches `:is_odd` (for example, in a Rust binary
    # that only calls `is_odd::is_odd`), that reference is a "backward
    # reference", which `lld --warn-backrefs` rejects.
    # `alwayslink` makes the linker always include `parity.cc`, so its
    # definitions are available before `:is_odd` is reached.
    alwayslink = True,
)

# The declarations in `parity.h`, without their definitions, so that `:is_odd`
# can call `IsEven` without depending on `:parity` (which depends on `:is_odd`).
# The definitions come from `:parity`, which every binary that uses `:is_odd`
# must also depend on.
cc_library(
    name = "parity_decls",
    hdrs = ["parity.h"],
    aspect_hints = [":parity_decls_rust.hint"],
    visibility = ["//visibility:private"],
)

rust_api_from_cpp(
    name = "parity_decls_rust",
    cpp_target = ":parity_decls",
)

# The Rust implementation of `IsOdd`.
rust_library(
    name = "is_odd",
    srcs = ["is_odd.rs"],
    edition = "2024",
    deps = [":parity_decls_rust"],
)

# The C++ API for `:is_odd`, which `parity.cc` calls.
cpp_api_from_rust(
    name = "is_odd_cc",
    crate = ":is_odd",
)

cc_binary(
    name = "main_cc",
    srcs = ["main.cc"],
    deps = [":parity"],
)

rust_binary(
    name = "main_rs",
    srcs = ["main.rs"],
    edition = "2024",
    deps = [
        ":is_odd",
        ":parity",
    ],
)

rust_test(
    name = "parity_test",
    srcs = ["parity_test.rs"],
    deps = [
        ":is_odd",
        ":parity",
        ":parity_decls_rust",
        "@crate_index//:googletest",
    ],
)
