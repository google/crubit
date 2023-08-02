# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect hint, to be attached to a `cc_library`, specifies the command line flag to be passed
to `rs_bindings_from_cc` when its Rust bindings are generated."""

visibility([
    "//rs_bindings_from_cc/bazel_support/...",
    "//rs_bindings_from_cc/test/bazel_unit_tests/cli_flag_aspect_hint_test/...",
    "//rs_bindings_from_cc/test/golden/...",
])

_RustBindingsFromCcCliFlagInfo = provider(
    doc = "The provider that specifies the command line flags and values for `rs_bindings_from_cc`.",
    fields = {
        "flags": "The command line flags and values for `rs_bindings_from_cc`.",
    },
)

def _rust_bindings_from_cc_cli_flag_impl(ctx):
    return [_RustBindingsFromCcCliFlagInfo(
        flags = ctx.attr.flags,
    )]

rust_bindings_from_cc_cli_flag = rule(
    attrs = {
        "flags": attr.string(
            doc = "The command line flags and values for `rs_bindings_from_cc`",
            mandatory = True,
        ),
    },
    implementation = _rust_bindings_from_cc_cli_flag_impl,
    doc = """
Defines an aspect hint that is used to pass command line flags to the `rs_bindings_from_cc` tool,
which affects the tool behavior when generating the Rust binding for the C++ target. This rule
should only be used by Crubit developers.
""",
)

def collect_rust_bindings_from_cc_cli_flags(_target, aspect_ctx):
    """Returns the command line flags and values for `rs_bindings_from_cc`.

    Args:
        _target: The target, as seen in aspect_hint.
        aspect_ctx: The ctx from an aspect_hint.

    Returns:
        A list of command line flags and values for `rs_bindings_from_cc`. The list is empty if no
        command line flag is specified.
    """
    flags = []
    for hint in aspect_ctx.rule.attr.aspect_hints:
        if _RustBindingsFromCcCliFlagInfo in hint:
            flags.append(hint[_RustBindingsFromCcCliFlagInfo].flags)
    return flags
