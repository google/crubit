# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A rule that collects C++ and Rust dependencies for the generated bindings files.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:providers.bzl",
    "CrateInfo",
    "DepInfo",
    "DepVariantInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "DepsForBindingsInfo",
)

def _deps_for_bindings_impl(ctx):
    dep_variant_infos = [
        DepVariantInfo(
            crate_info = dep[CrateInfo] if CrateInfo in dep else None,
            dep_info = dep[DepInfo] if DepInfo in dep else None,
            cc_info = dep[CcInfo] if CcInfo in dep else None,
            build_info = None,
        )
        for dep in ctx.attr.deps_for_generated_rs_file
    ]

    return [
        DepsForBindingsInfo(
            deps_for_rs_file = dep_variant_infos,
            deps_for_cc_file = [dep[CcInfo] for dep in ctx.attr.deps_for_generated_cc_file],
        ),
    ]

deps_for_bindings = rule(
    attrs = {
        "deps_for_generated_rs_file": attr.label_list(
            doc = "Rust dependencies that are needed to compile the generated _impl.rs file.",
            default = [],
        ),
        "deps_for_generated_cc_file": attr.label_list(
            doc = "C++ dependencies that are needed to compile the generated .cc file.",
            default = [],
        ),
    },
    implementation = _deps_for_bindings_impl,
)
