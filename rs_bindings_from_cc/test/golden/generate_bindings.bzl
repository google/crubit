# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A rule that generates bindings source files for a given C++ library."""

load(
    "//bazel_support:rust_bindings_from_cc_aspect.bzl",
    "GeneratedBindingsInfo",
    "rust_bindings_from_cc_aspect",
)

def _generate_bindings_impl(ctx):
    if not GeneratedBindingsInfo in ctx.attr.cc_library:
        fail("Bindings were not generated for the given cc_library.")
    bindings = ctx.attr.cc_library[GeneratedBindingsInfo]
    return OutputGroupInfo(
        cc_file = [bindings.cc_file],
        rust_file = [bindings.rust_file],
    )

generate_bindings = rule(
    attrs = {
        "cc_library": attr.label(providers = [CcInfo], aspects = [rust_bindings_from_cc_aspect]),
    },
    implementation = _generate_bindings_impl,
)
