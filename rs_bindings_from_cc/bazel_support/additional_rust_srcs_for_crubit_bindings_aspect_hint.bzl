# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect hint, to be attached to a `cc_library`, specifies additional Rust source files to be
included in the generated Rust crate."""

load("@bazel_skylib//lib:collections.bzl", "collections")

visibility([
    "//rs_bindings_from_cc/...",
    "//support/...",
])

AdditionalRustSrcsProviderInfo = provider(
    doc = """
The provider that specifies the Rust source files to be included in the Rust crate along with
generated Rust bindings of this C++ target.
""",
    fields = {
        "srcs": "The Rust source files to be included in addition to generated Rust bindings.",
        "namespace_path": "The namespace path for the Rust source files.",
    },
)

def _additional_rust_srcs_for_crubit_bindings_impl(ctx):
    return [AdditionalRustSrcsProviderInfo(
        srcs = ctx.attr.srcs,
        namespace_path = ctx.attr.namespace_path,
    )]

additional_rust_srcs_for_crubit_bindings = rule(
    attrs = {
        "srcs": attr.label_list(
            doc = "The Rust source files to be incldued in addition to generated Rust bindings.",
            allow_files = True,
            mandatory = True,
        ),
        "namespace_path": attr.string(
            doc = """This allows Rust source files define new entries inside of a specific existing C++ namespace instead of the top level namespace.
For modules which are not existing namespace names, use `pub mod` statement in the Rust source file instead.""",
            mandatory = False,
            default = "",
        ),
    },
    implementation = _additional_rust_srcs_for_crubit_bindings_impl,
    doc = """
Defines an aspect hint that is used to pass extra Rust source files to `rs_bindings_from_cc` tool's
`extra_rs_srcs` CLI argument.
""",
)

def get_additional_rust_srcs(_target, aspect_ctx):
    """Returns `extra_rs_srcs` associated with the `_target`.

    Args:
        _target: The target, as seen in aspect_hint.
        aspect_ctx: The ctx from an aspect_hint.

    Returns:
        A list of `File` and its module paths as specified by the `extra_rs_srcs` associated with the `_target`.
    """
    additional_rust_srcs = []
    for hint in aspect_ctx.rule.attr.aspect_hints:
        if AdditionalRustSrcsProviderInfo in hint:
            ns_path = hint[AdditionalRustSrcsProviderInfo].namespace_path
            for target in hint[AdditionalRustSrcsProviderInfo].srcs:
                srcs = [(f, ns_path) for f in target.files.to_list()]
                additional_rust_srcs.extend(srcs)
    return collections.uniq(additional_rust_srcs)
