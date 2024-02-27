# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for rust_bindings_from_cc_aspect."""

load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_flavor_transition",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)

OutputsInfo = provider(
    doc = """A helper provider for determining the command line. """,
    fields = {
        "args": "Args object",
    },
)

def _get_args_aspect_impl(target, ctx):
    bindings_action = None

    # Find CppHeaderAnalysis action
    for action in target.actions:
        if action.mnemonic == "CppHeaderAnalysis":
            bindings_action = action
            break

    args = bindings_action.args[0]
    return [OutputsInfo(args = args)]

get_args_aspect = aspect(
    implementation = _get_args_aspect_impl,
    attr_aspects = ["deps"],
    requires = [rust_bindings_from_cc_aspect],
)

def _cc_std_test_impl(ctx):
    out = ctx.actions.declare_file(ctx.label.name + "_test.sh")
    dep = ctx.attr.dep[0]
    ctx.actions.run(
        inputs = [dep[RustBindingsFromCcInfo].dep_variant_info.crate_info.output],
        arguments = [out.path, dep[OutputsInfo].args],
        executable = ctx.executable._include_directives_checker,
        outputs = [out],
    )
    return [DefaultInfo(executable = out)]

cc_std_test = rule(
    implementation = _cc_std_test_impl,
    attrs = {
        "dep": attr.label(
            aspects = [rust_bindings_from_cc_aspect, get_args_aspect],
            cfg = crubit_flavor_transition,
        ),
        "_include_directives_checker": attr.label(
            default = "//rs_bindings_from_cc/test/bazel_unit_tests/cc_std:check_include_directives",
            executable = True,
            cfg = "exec",
        ),
    },
    test = True,
)
