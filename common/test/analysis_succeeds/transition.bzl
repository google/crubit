# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Rule to transition to an unsupported platform for Crubit."""

def _unsupported_by_crubit_transition_impl(_settings, _attr):
    return {
        "//command_line_option:platforms": "//common/test/analysis_succeeds:unsupported_by_crubit_platform",
    }

_unsupported_by_crubit_transition = transition(
    implementation = _unsupported_by_crubit_transition_impl,
    inputs = [],
    outputs = [
        "//command_line_option:platforms",
    ],
)

def _unsupported_by_crubit_impl(_ctx):
    return []

unsupported_by_crubit = rule(
    implementation = _unsupported_by_crubit_impl,
    cfg = _unsupported_by_crubit_transition,
)
