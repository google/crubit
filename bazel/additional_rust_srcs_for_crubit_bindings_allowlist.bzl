# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Packages allowed to load `additional_rust_srcs_for_crubit_bindings`.

`additional_rust_srcs_for_crubit_bindings` is an allowlisted API. To use it,
add your package to this list (keep it sorted). The list is shared by the
public entry point, `//bazel:additional_rust_srcs_for_crubit_bindings.bzl`,
and by its implementation.
"""

ADDITIONAL_RUST_SRCS_FOR_CRUBIT_BINDINGS_USERS = [
    # <internal link> start
    "//cc_bindings_from_rs/...",
    "//examples/...",
    "//rs_bindings_from_cc/...",
    "//support/...",
    # <internal link> end
]
