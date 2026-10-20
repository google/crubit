# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Re-exports Crubit toolchain definitions."""

load(
    "//cc_bindings_from_rs/bazel_support:toolchain.bzl",
    _cc_bindings_from_rs_toolchain = "cc_bindings_from_rs_toolchain",
)
load(
    "//rs_bindings_from_cc/bazel_support:toolchain.bzl",
    _rs_bindings_from_cc_toolchain = "rs_bindings_from_cc_toolchain",
)

rs_bindings_from_cc_toolchain = _rs_bindings_from_cc_toolchain
cc_bindings_from_rs_toolchain = _cc_bindings_from_rs_toolchain
