# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A macro for generating tests with various Crubit build flavors."""

load("@rules_rust//rust:defs.bzl", "rust_test")

def crubit_rust_test(
        name,
        **kwargs):
    """A wrapper for `rust_test` for Google-internal purposes."""
    rust_test(
        name = name,
        **kwargs
    )
