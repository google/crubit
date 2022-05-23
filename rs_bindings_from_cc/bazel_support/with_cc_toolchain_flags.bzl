# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Creates a wrapper shell script that passes all the clang flags for header parsing.

This rule simplifies experimenting with the binary.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")

DISABLED_FEATURES = ["module_maps"]

def _append_escaped_newline(sequence):
    return [a + " \\\n" for a in sequence]

def get_cc_command_line_for_action(ctx, action_name):
    """Returns the command line flags for the given cc action name.

    Args:
      ctx: the context.
      action_name: One of the cc action names.

    Returns:
      The command line flags for the given action.
    """

    cc_toolchain = find_cpp_toolchain(ctx)
    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
        requested_features = ctx.features,
        unsupported_features = DISABLED_FEATURES + ctx.disabled_features,
    )
    stl = ctx.attr._stl[CcInfo].compilation_context
    variables = cc_common.create_compile_variables(
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        user_compile_flags = ctx.fragments.cpp.copts + ctx.fragments.cpp.cxxopts,
        system_include_directories = depset(cc_toolchain.built_in_include_directories, transitive = [stl.system_includes]),
        include_directories = stl.includes,
        quote_include_directories = stl.quote_includes,
    )
    return cc_common.get_memory_inefficient_command_line(
        feature_configuration = feature_configuration,
        action_name = action_name,
        variables = variables,
    )

def _with_cc_toolchain_flags_impl(ctx):
    command_line = get_cc_command_line_for_action(ctx, ACTION_NAMES.cpp_header_parsing)
    driver = ctx.actions.declare_file(ctx.attr.name)
    ctx.actions.write(
        is_executable = True,
        content = """
#!/bin/bash

set -euo pipefail
# If invoked via `bazel run`, go back to the source tree.
[[ -n "$BUILD_WORKING_DIRECTORY" ]] && cd "$BUILD_WORKING_DIRECTORY"

{} \\
  {} "${}" -- \\
  {} \\
  ;
""".format(
            ctx.file.binary.path,
            "  ".join(_append_escaped_newline(ctx.attr.extra_args)),
            "{@}",
            "  ".join(_append_escaped_newline(command_line)),
        ),
        output = driver,
    )

    runfiles = ctx.runfiles(
        files = [
            ctx.file.binary,
        ],
    )

    return [
        DefaultInfo(executable = driver, runfiles = runfiles),
    ]

with_cc_toolchain_flags = rule(
    attrs = {
        "binary": attr.label(
            allow_single_file = True,
            doc = """
                Executable StandaloneClangTool binary that will
                be invoked with all clang flags for header-parsing.""",
        ),
        "extra_args": attr.string_list(
            default = ["--use_tool_args_for_compile"],
            doc = "Additional flags to be passed right after the binary.",
        ),
        "_cc_toolchain": attr.label(
            default = Label("@bazel_tools//tools/cpp:current_cc_toolchain"),
        ),
        "_stl": attr.label(default = Label("//third_party/stl")),
    },
    executable = True,
    fragments = ["cpp"],
    host_fragments = ["cpp"],
    toolchains = [
        "@rules_rust//rust:toolchain",
        "@bazel_tools//tools/cpp:toolchain_type",
    ],
    implementation = _with_cc_toolchain_flags_impl,
)
