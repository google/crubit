# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Macros and rules for running Crubit interop tests under different standard library flavors."""

load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

_FORWARDED_TEST_ATTRS = ("tags", "size", "timeout", "flaky", "shard_count", "local", "args", "env")

_STDLIB_SETTING = "//support/rs_std:stdlib"

def _stdlib_transition_impl(_settings, attr):
    return {
        _STDLIB_SETTING: attr.stdlib,
    }

_stdlib_transition = transition(
    implementation = _stdlib_transition_impl,
    inputs = [],
    outputs = [_STDLIB_SETTING],
)

def _crubit_stdlib_cc_test_wrapper_impl(ctx):
    target = ctx.attr.target[0]
    actual_exe = target[DefaultInfo].files_to_run.executable
    executable = ctx.outputs.executable
    ctx.actions.symlink(
        output = executable,
        target_file = actual_exe,
        is_executable = True,
    )
    providers = [
        DefaultInfo(
            executable = executable,
            runfiles = target[DefaultInfo].default_runfiles.merge(ctx.runfiles([executable])),
            files = depset([executable]),
        ),
    ]
    if testing.ExecutionInfo in target:
        providers.append(target[testing.ExecutionInfo])
    if RunEnvironmentInfo in target:
        providers.append(target[RunEnvironmentInfo])
    if ctx.attr.env:
        providers.append(testing.TestEnvironment(ctx.attr.env))
    if InstrumentedFilesInfo in target:
        providers.append(target[InstrumentedFilesInfo])
    return providers

crubit_stdlib_cc_test_wrapper_test = rule(
    implementation = _crubit_stdlib_cc_test_wrapper_impl,
    test = True,
    attrs = {
        "target": attr.label(
            mandatory = True,
            cfg = _stdlib_transition,
            doc = "The test target to build under the stdlib transition and execute.",
        ),
        "stdlib": attr.label(
            mandatory = True,
            doc = "The standard library flavor target under which to build and run the test.",
        ),
        "env": attr.string_dict(
            doc = "Environment variables for the test.",
        ),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
)

def crubit_stdlib_cc_test(
        name,
        stdlib,
        srcs = None,
        deps = None,
        target = None,
        **kwargs):
    """Builds and runs a Crubit interop C++ test under a specific standard library flavor.

    Args:
      name: The base name of the test target (or full test name if target is specified).
      stdlib: The standard library flavor name ("alloc", "core", "std") or target label.
      srcs: Optional source files for standalone test instantiation.
      deps: Optional dependencies for standalone test instantiation.
      target: Optional test target to transition. Defaults to `":" + name` if srcs/deps not specified.
      **kwargs: Additional rule arguments.
    """
    if stdlib in ("alloc", "core", "std"):
        stdlib_target = "//support/rs_std:" + stdlib
        stdlib_suffix = stdlib
    else:
        stdlib_target = stdlib if (stdlib.startswith(":") or stdlib.startswith("//") or stdlib.startswith("@")) else ":" + stdlib
        stdlib_suffix = stdlib.lstrip("/:@").replace("/", "_").replace(":", "_")

    if target == None:
        if srcs != None or deps != None:
            raw_target_name = "_" + name + "_underlying"
            raw_kwargs = dict(kwargs)
            raw_kwargs["tags"] = raw_kwargs.get("tags", []) + ["manual"]
            crubit_cc_test(
                name = raw_target_name,
                srcs = srcs,
                deps = deps,
                **raw_kwargs
            )
            target = ":" + raw_target_name
            test_name = name
        else:
            target = ":" + name
            test_name = name + "_on_" + stdlib_suffix
    else:
        test_name = name

    test_kwargs = {attr: kwargs[attr] for attr in _FORWARDED_TEST_ATTRS if attr in kwargs}
    crubit_stdlib_cc_test_wrapper_test(
        name = test_name,
        target = target,
        stdlib = stdlib_target,
        **test_kwargs
    )
