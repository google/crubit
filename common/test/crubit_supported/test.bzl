# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Analysis test verifying `crubit_supported` selects correctly across platforms."""

def _check_expected_file_impl(ctx):
    if len(ctx.files.src) != 1:
        fail("Expected exactly 1 file in src, got: %s" % ctx.files.src)
    actual = ctx.files.src[0].basename
    if actual != ctx.attr.expected_file:
        fail("Expected %s on this platform, but got %s" % (ctx.attr.expected_file, actual))
    return []

check_expected_file = rule(
    implementation = _check_expected_file_impl,
    attrs = {
        "src": attr.label(mandatory = True, allow_files = True),
        "expected_file": attr.string(mandatory = True),
    },
)

def _unsupported_transition_impl(_settings, _attr):
    return {
        "//command_line_option:platforms": "//common/test/crubit_supported:unsupported_platform",
    }

_unsupported_transition = transition(
    implementation = _unsupported_transition_impl,
    inputs = [],
    outputs = ["//command_line_option:platforms"],
)

check_unsupported_expected_file = rule(
    implementation = _check_expected_file_impl,
    cfg = _unsupported_transition,
    attrs = {
        "src": attr.label(mandatory = True, allow_files = True),
        "expected_file": attr.string(mandatory = True),
    },
)
