# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
Generates Rust bindings for a C++ target such as a `cc_library`.

This acts like a `rust_library`, and may be listed in `deps` of a rust rule.
The C++ target must still enable bindings generation via `aspect_hints`.

By default, `bazel build` will compile the bindings.
You can request bindings generation only via `--output_groups=sources`.
"""

load("@rules_rust//rust:rust_common.bzl", "CrateInfo", "DepInfo")
load("//cc_bindings_from_rs/bazel_support:providers.bzl", "CcBindingsFromRustInfo")
load("@@//rs_bindings_from_cc/bazel_support:providers.bzl", "GeneratedBindingsInfo", "RustBindingsFromCcInfo")
load("@@//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl", "rust_bindings_from_cc_aspect")

def _rust_api_from_cpp_generate_bindings_impl(ctx):
    """Implementation of the `rust_bindings_from_cc` rule."""

    variant = ctx.attr.target[RustBindingsFromCcInfo].dep_variant_info
    if not variant or not OutputGroupInfo in ctx.attr.target:
        fail("No bindings generated for target: %s" % ctx.attr.target.label)
    providers = [
        variant.crate_info,
        variant.dep_info,
        variant.cc_info,
        DefaultInfo(files = ctx.attr.target[OutputGroupInfo]["out_compiled"]),
        OutputGroupInfo(sources = ctx.attr.target[OutputGroupInfo]["out"]),
    ]
    if GeneratedBindingsInfo in ctx.attr.target:
        providers.append(ctx.attr.target[GeneratedBindingsInfo])
    if CcBindingsFromRustInfo in ctx.attr.target:
        providers.append(ctx.attr.target[CcBindingsFromRustInfo])
    return providers

_rust_api_from_cpp_generate_bindings = rule(
    implementation = _rust_api_from_cpp_generate_bindings_impl,
    doc = "Generates Rust bindings for a C++ target.",
    provides = [CrateInfo, DepInfo],
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

def rust_api_from_cpp(name, cpp_target):
    """Generates Rust bindings for a C++ target.

    Args:
      name: The name of the rule.
      cpp_target: The C++ target to generate bindings for.
    """
    _rust_api_from_cpp_generate_bindings(
        name = name,
        target = cpp_target,
    )
