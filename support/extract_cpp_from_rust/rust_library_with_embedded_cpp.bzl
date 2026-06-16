# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Macro to compile a Rust library with embedded C++ payload code."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//support/extract_cpp_from_rust:extract_cpp.bzl", "extract_cpp")

def rust_library_with_embedded_cpp(name, srcs, deps = [], cc_deps = [], **kwargs):
    """Compiles a Rust library containing `global_cpp!` blocks."""
    cc_lib_name = name + "_extracted_cc"
    extracted_header = cc_lib_name + ".h"

    extract_cpp(
        name = cc_lib_name + "_extract_cpp",
        srcs = srcs,
        out = extracted_header,
    )

    cc_library(
        name = cc_lib_name,
        hdrs = [extracted_header],
        deps = cc_deps,
        aspect_hints = ["//features:supported"],
    )

    rust_bindings_name = name + "_rust_bindings"
    rust_api_from_cpp(
        name = rust_bindings_name,
        cpp_target = ":" + cc_lib_name,
    )

    rust_library(
        name = name,
        srcs = srcs,
        deps = deps + [":" + rust_bindings_name],
        **kwargs
    )
