# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Test-only bindings generation macros."""

def crubit_test_cc_library(**kwargs):
    """A wrapper for cc_library in Crubit integration tests.

    This is equivalent to cc_library, but it sets the default aspect_hints to `:experimental`.
    """
    kwargs.setdefault("aspect_hints", ["//third_party/crubit:experimental"])
    native.cc_library(
        **kwargs
    )
