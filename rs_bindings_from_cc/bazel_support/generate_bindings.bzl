# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")

def _get_hdrs_command_line(hdrs):
    return ["--public_headers=" + ",".join([x.path for x in hdrs])]

def _get_extra_rs_srcs_command_line(extra_rs_srcs):
    return ["--extra_rs_srcs=" + ",".join([x.path for x in extra_rs_srcs])]

def generate_bindings(
        ctx,
        attr,
        cc_toolchain,
        feature_configuration,
        compilation_context,
        public_hdrs,
        header_includes,
        action_inputs,
        target_args,
        extra_rs_srcs,
        extra_rs_bindings_from_cc_cli_flags):
    """Runs the bindings generator.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      cc_toolchain: The cc_toolchain.
      feature_configuration: The feature configuration.
      compilation_context: The compilation context for this action.
      public_hdrs: A list of headers to be passed to the tool via the "--public_headers" flag.
      header_includes: A list of flags to be passed to the command line with "-include".
      action_inputs: A depset of inputs to the bindings generating action.
      target_args: A depset of strings, each one representing mapping of target to
                        its per-target arguments (headers, features) in json format.
      extra_rs_srcs: A list of extra source files to add.
      extra_rs_bindings_from_cc_cli_flags: CLI flags to be passed to `rs_bindings_from_cc`.

    Returns:
      tuple(cc_output, rs_output, namespaces_output, error_report_output): The generated source files.
    """
    cc_output = ctx.actions.declare_file(ctx.label.name + "_rust_api_impl.cc")
    rs_output = ctx.actions.declare_file(ctx.label.name + "_rust_api.rs")
    namespaces_output = ctx.actions.declare_file(ctx.label.name + "_namespaces.json")
    error_report_output = None

    rs_bindings_from_cc_flags = [
        "--stderrthreshold=2",
        "--target=" + str(ctx.label),
        "--rs_out",
        rs_output.path,
        "--cc_out",
        cc_output.path,
        "--namespaces_out",
        namespaces_output.path,
        "--crubit_support_path",
        "support",
        "--clang_format_exe_path",
        ctx.file._clang_format.path,
        "--rustfmt_exe_path",
        ctx.file._rustfmt.path,
        "--rustfmt_config_path",
        ctx.file._rustfmt_cfg.path,
    ] + extra_rs_bindings_from_cc_cli_flags
    if ctx.attr._generate_error_report[BuildSettingInfo].value:
        error_report_output = ctx.actions.declare_file(ctx.label.name + "_rust_api_error_report.json")
        rs_bindings_from_cc_flags += [
            "--error_report_out",
            error_report_output.path,
        ]

    clang_builtin_headers = "third_party/llvm/llvm-project/clang/lib/Headers"
    variables = cc_common.create_compile_variables(
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        system_include_directories = depset(
            direct = [
                cc_toolchain.built_in_include_directories[0],
                # Clang's builtin headers:
                clang_builtin_headers,
                # Fuzzer and sanitizer headers:
                "third_party/llvm/llvm-project/compiler-rt/include",
                cc_toolchain.built_in_include_directories[2],
                ],
            transitive = [compilation_context.system_includes],
        ),
        include_directories = compilation_context.includes,
        quote_include_directories = compilation_context.quote_includes,
        user_compile_flags = ctx.fragments.cpp.copts +
                             ctx.fragments.cpp.cxxopts +
                             header_includes + (
            attr.copts if hasattr(attr, "copts") else []
        ),
        preprocessor_defines = compilation_context.defines,
        variables_extension = {
            "rs_bindings_from_cc_tool": ctx.executable._generator.path,
            "rs_bindings_from_cc_flags": rs_bindings_from_cc_flags + _get_hdrs_command_line(public_hdrs) + _get_extra_rs_srcs_command_line(extra_rs_srcs),
            "target_args": target_args,
        },
    )

    # Run the `rs_bindings_from_cc` to generate the _rust_api_impl.cc and _rust_api.rs files.
    cc_common.create_compile_action(
        compilation_context = compilation_context,
        actions = ctx.actions,
        action_name = ACTION_NAMES.rs_bindings_from_cc,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        source_file = public_hdrs[0],
        output_file = cc_output,
        additional_inputs = depset(
            direct = [
                ctx.executable._clang_format,
                ctx.executable._rustfmt,
                ctx.executable._generator,
            ] + ctx.files._rustfmt_cfg + extra_rs_srcs,
            transitive = [action_inputs],
        ),
        additional_outputs = [x for x in [rs_output, namespaces_output, error_report_output] if x != None],
        variables = variables,
    )
    return (cc_output, rs_output, namespaces_output, error_report_output)
