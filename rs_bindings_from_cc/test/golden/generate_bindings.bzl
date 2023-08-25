# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A rule that generates bindings source files for a given C++ library."""

load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)
load(
    "//google_internal/build_flavors:crubit_build_flavors_dev.bzl",
    "crubit_flavor_transition",
)

def _generate_bindings_impl(ctx):
    cc_library = ctx.attr.cc_library[0]
    if not GeneratedBindingsInfo in cc_library:
        fail("Bindings were not generated for the given cc_library.")
    bindings = cc_library[GeneratedBindingsInfo]
    return OutputGroupInfo(
        cc_file = [bindings.cc_file],
        rust_file = [bindings.rust_file],
        namespaces_file = [bindings.namespaces_file],
    )

generate_bindings = rule(
    attrs = {
        "cc_library": attr.label(
            providers = [CcInfo],
            aspects = [rust_bindings_from_cc_aspect],
            cfg = crubit_flavor_transition,
        ),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
    implementation = _generate_bindings_impl,
)
