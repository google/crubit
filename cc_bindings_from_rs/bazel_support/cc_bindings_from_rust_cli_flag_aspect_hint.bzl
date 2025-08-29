# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect hint, to be attached to a `rust_library`, specifies the command line flag to be passed
to `cc_bindings_from_rust` when its Rust bindings are generated."""

visibility([
    "//cc_bindings_from_rs/bazel_support/...",
    "//cc_bindings_from_rs/test/...",
])

_CcBindingsFromRustCliFlagInfo = provider(
    doc = "The provider that specifies the command line flags and values for `cc_bindings_from_rust`.",
    fields = {
        "flags": "The command line flags and values for `cc_bindings_from_rust`.",
    },
)

def _cc_bindings_from_rust_cli_flag_impl(ctx):
    return [_CcBindingsFromRustCliFlagInfo(
        flags = ctx.attr.flags,
    )]

cc_bindings_from_rust_cli_flag = rule(
    attrs = {
        "flags": attr.string(
            doc = "The command line flags and values for `cc_bindings_from_rust`",
            mandatory = True,
        ),
    },
    implementation = _cc_bindings_from_rust_cli_flag_impl,
    doc = """
Defines an aspect hint that is used to pass command line flags to the `cc_bindings_from_rust` tool,
which affects the tool behavior when generating the C++ binding for the Rust target. This rule
should only be used by Crubit developers.
""",
)

def collect_cc_bindings_from_rust_cli_flags(_target, aspect_ctx):
    """Returns the command line flags and values for `cc_bindings_from_rust`.

    Args:
        _target: The target, as seen in aspect_hint.
        aspect_ctx: The ctx from an aspect_hint.

    Returns:
        A list of command line flags and values for `cc_bindings_from_rust`. The list is empty if no
        command line flag is specified.
    """
    flags = []
    for hint in aspect_ctx.rule.attr.aspect_hints:
        if _CcBindingsFromRustCliFlagInfo in hint:
            flags.append(hint[_CcBindingsFromRustCliFlagInfo].flags)
    return flags
