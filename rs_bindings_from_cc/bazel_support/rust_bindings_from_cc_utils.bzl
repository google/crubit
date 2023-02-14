# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load("//rs_bindings_from_cc/bazel_support:compile_cc.bzl", "compile_cc")
load("//rs_bindings_from_cc/bazel_support:compile_rust.bzl", "compile_rust")
load("//rs_bindings_from_cc/bazel_support:generate_bindings.bzl", "generate_bindings")
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
    "RustBindingsFromCcInfo",
)

def generate_and_compile_bindings(
        ctx,
        attr,
        compilation_context,
        public_hdrs,
        header_includes,
        action_inputs,
        targets_and_headers,
        extra_rs_srcs,
        deps_for_cc_file,
        deps_for_rs_file,
        extra_cc_compilation_action_inputs = []):
    """Runs the bindings generator.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      compilation_context: The current compilation context.
      public_hdrs: A list of headers to be passed to the tool via the "--public_headers" flag.
      header_includes: A list of flags to be passed to the command line with "-include".
      action_inputs: A depset of inputs to the bindings generating action.
      targets_and_headers: A depset of strings, each one representing mapping of target to " +
                          "its headers in json format.
      extra_rs_srcs: list[file]: Additional source files for the Rust crate.
      deps_for_cc_file: list[CcInfo]: CcInfos needed by the generated C++ source file.
      deps_for_rs_file: list[DepVariantInfo]: DepVariantInfos needed by the generated Rust source file.
      extra_cc_compilation_action_inputs: A list of input files for the C++ compilation action.

    Returns:
      A RustBindingsFromCcInfo containing the result of the compilation of the generated source
      files, as well a GeneratedBindingsInfo provider containing the generated source files.
    """
    cc_toolchain = find_cpp_toolchain(ctx)

    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
        requested_features = ctx.features,
        unsupported_features = ctx.disabled_features + ["module_maps"],
    )

    cc_output, rs_output, namespaces_output, error_report_output = generate_bindings(
        ctx = ctx,
        attr = attr,
        cc_toolchain = cc_toolchain,
        feature_configuration = feature_configuration,
        compilation_context = compilation_context,
        public_hdrs = public_hdrs,
        header_includes = header_includes,
        action_inputs = action_inputs,
        targets_and_headers = targets_and_headers,
        extra_rs_srcs = extra_rs_srcs,
    )

    # Relocate the rs files so that they can be read by rustc using relative paths.
    extra_rs_srcs_relocated = []
    for file in extra_rs_srcs:
        new_file = ctx.actions.declare_file(file.path, sibling = rs_output)
        ctx.actions.symlink(output = new_file, target_file = file)
        extra_rs_srcs_relocated.append(new_file)

    # Compile the "_rust_api_impl.cc" file
    cc_info = compile_cc(
        ctx,
        attr,
        cc_toolchain,
        feature_configuration,
        cc_output,
        deps_for_cc_file,
        extra_cc_compilation_action_inputs,
    )

    # Compile the "_rust_api.rs" file together with extra_rs_srcs.
    dep_variant_info = compile_rust(
        ctx,
        attr,
        rs_output,
        extra_rs_srcs_relocated,
        deps_for_rs_file,
    )

    return [
        RustBindingsFromCcInfo(
            cc_info = cc_info,
            dep_variant_info = dep_variant_info,
            targets_and_headers = targets_and_headers,
            namespaces = namespaces_output,
        ),
        GeneratedBindingsInfo(
            cc_file = cc_output,
            rust_file = rs_output,
            namespaces_file = namespaces_output,
        ),
        OutputGroupInfo(out = depset([x for x in [cc_output, rs_output, namespaces_output, error_report_output] if x != None])),
    ]

bindings_attrs = {
    "_cc_toolchain": attr.label(
        default = "@bazel_tools//tools/cpp:current_cc_toolchain",
    ),
    "_generator": attr.label(
        default = "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_target",
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_deps_for_bindings": attr.label(
        doc = "Dependencies that are needed to compile the generated .cc and .rs file.",
        default = "//rs_bindings_from_cc/bazel_support:deps_for_bindings",
    ),
    "_grep_includes": attr.label(
        allow_single_file = True,
        default = Label("@bazel_tools//tools/cpp:grep-includes"),
        cfg = "exec",
    ),
    "_clang_format": attr.label(
        default = "//third_party/crosstool/google3_users:stable_clang-format",
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_rustfmt": attr.label(
        default = "//nowhere/llvm/rust:genrustfmt_for_crubit_aspects",
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_rustfmt_cfg": attr.label(
        default = "//nowhere:rustfmt.toml",
        allow_single_file = True,
    ),
    # TODO(hlopko): Either 1) remove the unneeded `_error_format` and
    # `_extra_rustc_flags` attributes below *or* 2) actually start using them
    # (both for `rs_bindings_from_cc` and for `cc_bindings_from_rs`).
    "_error_format": attr.label(
        default = "@rules_rust//:error_format",
    ),
    "_extra_rustc_flags": attr.label(
        default = "@rules_rust//:extra_rustc_flags",
    ),
    "_process_wrapper": attr.label(
        default = "@rules_rust//util/process_wrapper",
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_builtin_hdrs": attr.label(
        default = "//rs_bindings_from_cc:builtin_headers",
    ),
    "_generate_error_report": attr.label(
        default = "//rs_bindings_from_cc/bazel_support:generate_error_report",
    ),
}
