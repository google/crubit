# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains helper method to test cc_bindings_from_rs aspect."""

load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust_aspect",
)
load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    "CcBindingsFromRustInfo",
)

ActionsInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.cc` and `.rs` files."),
    fields = {"actions": "List[Action]: actions registered by the underlying target."},
)

def _attach_aspect_impl(ctx):
    return [ctx.attr.dep[CcBindingsFromRustInfo], ActionsInfo(actions = ctx.attr.dep.actions)]

attach_aspect = rule(
    implementation = _attach_aspect_impl,
    attrs = {
        "dep": attr.label(aspects = [cc_bindings_from_rust_aspect]),
    },
)
