# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""Common utilities for Crubit unit tests."""

load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
    "RustBindingsFromCcInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)

ActionsInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.cc` and `.rs` files."),
    fields = {"actions": "List[Action]: actions registered by the underlying target."},
)

def _attach_aspect_impl(ctx):
    providers = [ctx.attr.dep[RustBindingsFromCcInfo], ActionsInfo(actions = ctx.attr.dep.actions)]
    if GeneratedBindingsInfo in ctx.attr.dep:
        providers.append(ctx.attr.dep[GeneratedBindingsInfo])
    return providers

# Runs `rust_bindings_from_cc_aspect` on `dep` and re-exports the providers it returns, so that
# analysis tests can read them from this target.
attach_aspect = rule(
    implementation = _attach_aspect_impl,
    attrs = {
        "dep": attr.label(aspects = [rust_bindings_from_cc_aspect]),
    },
)
