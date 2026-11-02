# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect that generates C++ bindings for Rust targets (`cpp_api_from_rust_aspect`).

Usable on the command line, e.g.
`--aspects=//bazel:cpp_api_from_rust_aspect.bzl%cpp_api_from_rust_aspect`.
"""

load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    _cc_bindings_from_rust_aspect = "cc_bindings_from_rust_aspect",
)

cpp_api_from_rust_aspect = _cc_bindings_from_rust_aspect
