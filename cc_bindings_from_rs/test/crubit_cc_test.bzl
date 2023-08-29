# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A macro for generating tests with various Crubit build flavors."""

load("@bazel_skylib//lib:new_sets.bzl", "sets")

def crubit_cc_test(
        name,
        **kwargs):
    """A wrapper for `cc_test` for Google-internal purposes."""
    native.cc_test(
        name = name,
        **kwargs
    )
