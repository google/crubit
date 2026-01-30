# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
Rule to provide a named target for generated bindings providers.

This rule does not itself produce bindings. It has no output groups, but a dependent rule could extract the bindings and use them or output them itself.
"""

load("@@//rs_bindings_from_cc/bazel_support:providers.bzl", "RustBindingsFromCcInfo")
load("@@//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl", "rust_bindings_from_cc_aspect")

def _rust_bindings_from_cc_impl(ctx):
    """Implementation of the `rust_bindings_from_cc` rule."""
    return [ctx.attr.target[RustBindingsFromCcInfo]]

rust_bindings_from_cc = rule(
    implementation = _rust_bindings_from_cc_impl,
    doc = "Generates Rust bindings for a C++ target.",
    attrs = {
        "target": attr.label(
            doc = "The C++ target to generate bindings for.",
            allow_files = False,
            mandatory = True,
            providers = [RustBindingsFromCcInfo],
            aspects = [rust_bindings_from_cc_aspect],
        ),
    },
)
