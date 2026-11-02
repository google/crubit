# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`additional_rust_srcs_for_crubit_bindings` aspect hint (allowlisted API).

To use it, add your package to `additional_rust_srcs_for_crubit_bindings_allowlist.bzl`.
"""

load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    _additional_rust_srcs_for_crubit_bindings = "additional_rust_srcs_for_crubit_bindings",
)
load(
    ":additional_rust_srcs_for_crubit_bindings_allowlist.bzl",
    "ADDITIONAL_RUST_SRCS_FOR_CRUBIT_BINDINGS_USERS",
)

visibility(ADDITIONAL_RUST_SRCS_FOR_CRUBIT_BINDINGS_USERS)

additional_rust_srcs_for_crubit_bindings = _additional_rust_srcs_for_crubit_bindings
