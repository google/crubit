# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cpp_api_from_rust` rule."""

load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    _cc_bindings_from_rust = "cc_bindings_from_rust",
)

cpp_api_from_rust = _cc_bindings_from_rust
