# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")

def _get_hdrs_command_line(hdrs):
    return ["--public_headers=" + ",".join([x.path for x in hdrs])]

def generate_bindings(
        ctx,
        attr,
        cc_toolchain,
        feature_configuration,
        compilation_context,
        public_hdrs,
        header_includes,
        action_inputs,
        targets_and_headers):
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
      targets_and_headers: A depset of strings, each one representing mapping of target to " +
                          "its headers in json format.

    Returns:
      tuple(cc_output, rs_output): The generated source files.
    """
    cc_output = ctx.actions.declare_file(ctx.label.name + "_rust_api_impl.cc")
    rs_output = ctx.actions.declare_file(ctx.label.name + "_rust_api.rs")
    namespaces_output = ctx.actions.declare_file(ctx.label.name + "_namespaces.json")

    variables = cc_common.create_compile_variables(
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        system_include_directories = depset(
            direct = [
                cc_toolchain.built_in_include_directories[0],
                # Clang's builtin headers:
                "third_party/llvm/llvm-project/clang/lib/Headers",
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
            "rs_bindings_from_cc_flags": [
                "--stderrthreshold=2",
                "--rs_out",
                rs_output.path,
                "--cc_out",
                cc_output.path,
                "--namespaces_out",
                namespaces_output.path,
                "--crubit_support_path",
                "rs_bindings_from_cc/support",
                "--rustfmt_exe_path",
                "third_party/unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt",
                "--rustfmt_config_path",
                "nowhere/rustfmt.toml",
            ] + _get_hdrs_command_line(public_hdrs),
            "targets_and_headers": targets_and_headers,
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
        grep_includes = ctx.file._grep_includes,
        additional_inputs = depset(
            direct = [
                ctx.executable._rustfmt,
                ctx.executable._generator,
            ] + ctx.files._rustfmt_cfg,
            transitive = [action_inputs],
        ),
        additional_outputs = [rs_output, namespaces_output],
        variables = variables,
    )
    return (cc_output, rs_output, namespaces_output)
