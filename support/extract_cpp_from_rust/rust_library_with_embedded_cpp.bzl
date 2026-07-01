# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Macro to compile a Rust library with embedded C++ payload code."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//support/extract_cpp_from_rust:extract_cpp.bzl", "extract_cpp")

def rust_library_with_embedded_cpp(name, srcs, deps = [], deps_of_cc_library = [], **kwargs):
    """Compiles a Rust library containing `global_cpp!` or 'inline_cpp!' blocks."""
    cc_lib_name = name + "_extracted_cc"
    extracted_header = cc_lib_name + ".h"

    target = "//{}:{}".format(native.package_name(), name)

    extract_cpp(
        name = cc_lib_name + "_extract_cpp",
        srcs = srcs,
        out = extracted_header,
        target = target,
    )

    cc_library(
        name = cc_lib_name,
        hdrs = [extracted_header],
        deps = deps_of_cc_library,
        aspect_hints = ["//features:supported"],
    )

    rust_bindings_name = name + "_rust_bindings"
    rust_api_from_cpp(
        name = rust_bindings_name,
        cpp_target = ":" + cc_lib_name,
    )

    bindings_label = ":" + rust_bindings_name
    rust_library(
        name = name,
        srcs = srcs,
        deps = deps + [":" + rust_bindings_name],
        aliases = {
            bindings_label: "inline_cpp_generated_bindings",
            ":" + cc_lib_name: "inline_cpp_generated_bindings",
        },
        proc_macro_deps = kwargs.pop("proc_macro_deps", []) + [
            "//support/extract_cpp_from_rust:inline_cpp_macro",
        ],
        rustc_env = dict(kwargs.pop("rustc_env", {}), CRUBIT_TARGET = target),
        **kwargs
    )
