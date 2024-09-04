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

_AdditionalRustSrcsProviderInfo = provider(
    doc = """
The provider that specifies the Rust source files to be included in the Rust crate along with
generated Rust bindings of this C++ target.
""",
    fields = {
        "srcs": "The Rust source files to be included in addition to generated Rust bindings.",
    },
)

def _additional_rust_srcs_for_crubit_bindings_impl(ctx):
    return [_AdditionalRustSrcsProviderInfo(
        srcs = ctx.attr.srcs,
    )]

additional_rust_srcs_for_crubit_bindings = rule(
    attrs = {
        "srcs": attr.label_list(
            doc = "The Rust source files to be incldued in addition to generated Rust bindings.",
            allow_files = True,
            mandatory = True,
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
        A list of `File`s as specified by the `extra_rs_srcs` associated with the `_target`.
    """
    additional_rust_srcs = []
    for hint in aspect_ctx.rule.attr.aspect_hints:
        if _AdditionalRustSrcsProviderInfo in hint:
            for target in hint[_AdditionalRustSrcsProviderInfo].srcs:
                additional_rust_srcs.extend(target.files.to_list())
    return collections.uniq(additional_rust_srcs)
