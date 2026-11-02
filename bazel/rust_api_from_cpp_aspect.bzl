# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect that generates Rust bindings for C++ targets (`rust_api_from_cpp_aspect`).

Usable on the command line, e.g.
`--aspects=//bazel:rust_api_from_cpp_aspect.bzl%rust_api_from_cpp_aspect`.

Kept separate from `cpp_api_from_rust_aspect.bzl` on purpose: this file is loaded
by the Rust rules themselves, and the other direction's rule transitively loads
the Rust rules, so a combined file would form a load cycle.
"""

load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    _rust_bindings_from_cc_aspect = "rust_bindings_from_cc_aspect",
)

rust_api_from_cpp_aspect = _rust_bindings_from_cc_aspect
