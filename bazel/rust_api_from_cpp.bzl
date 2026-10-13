# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`rust_api_from_cpp` rule."""

load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    _rust_api_from_cpp = "rust_api_from_cpp",
)

rust_api_from_cpp = _rust_api_from_cpp
