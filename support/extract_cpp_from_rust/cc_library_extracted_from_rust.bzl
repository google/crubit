# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A library that extracts `global_cpp!` and `inline_cpp!` blocks from Rust files and makes them available in C++."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//support/extract_cpp_from_rust:extract_cpp.bzl", "extract_cpp")

def cc_library_extracted_from_rust(name, srcs, **kwargs):
    extracted_header = name + ".h"

    extract_cpp(
        name = name + "_extract_cpp",
        srcs = srcs,
        out = extracted_header,
    )

    cc_library(
        name = name,
        hdrs = [extracted_header],
        **kwargs
    )
