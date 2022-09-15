# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""Common utilities for Crubit unit tests."""

load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)

ActionsInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.cc` and `.rs` files."),
    fields = {"actions": "List[Action]: actions registered by the underlying target."},
)

def _attach_aspect_impl(ctx):
    return [ctx.attr.dep[RustBindingsFromCcInfo], ActionsInfo(actions = ctx.attr.dep.actions)]

attach_aspect = rule(
    implementation = _attach_aspect_impl,
    attrs = {
        "dep": attr.label(aspects = [rust_bindings_from_cc_aspect]),
    },
)
