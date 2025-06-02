# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A wrapper rule to enable tests with --fission."""

def _enable_fission_transition_impl(_settings, _attr):
    return {"//command_line_option:fission": "yes"}

_enable_fission_transition = transition(
    implementation = _enable_fission_transition_impl,
    inputs = [],
    outputs = ["//command_line_option:fission"],
)

def _enable_fission_test_impl(ctx):
    tut = ctx.attr.target_under_test[0]
    output = ctx.actions.declare_file(ctx.label.name)
    target = tut[DefaultInfo].files.to_list()[0]

    ctx.actions.symlink(output = output, target_file = target, is_executable = True)
    providers = [
        DefaultInfo(
            executable = output,
        ),
    ]
    if testing.ExecutionInfo in tut:
        providers.append(tut[testing.ExecutionInfo])
    return providers

enable_fission_test = rule(
    implementation = _enable_fission_test_impl,
    attrs = {
        "target_under_test": attr.label(mandatory = True, cfg = _enable_fission_transition, executable = True),
    },
    test = True,
)
