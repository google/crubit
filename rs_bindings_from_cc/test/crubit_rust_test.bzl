# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A macro for generating tests with various Crubit build flavors."""

load("@rules_rust//rust:defs.bzl", "rust_test")

This mostly exists to make a Google-internal process easier. OSS developers can just use
rust_test.
def crubit_rust_test(**kwargs):
    rust_test(**kwargs)
