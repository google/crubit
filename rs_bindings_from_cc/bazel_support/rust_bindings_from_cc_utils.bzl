# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:common.bzl", "rust_common")

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:providers.bzl", "DepVariantInfo")

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:rustc.bzl", "rustc_compile_action")
load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")

RustBindingsFromCcInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.cc` and `.rs` files."),
    fields = {
        "cc_info": "A CcInfo provider for the implementation of the API projection.",
        "dep_variant_info": ("A DepVariantInfo provider that carries information from the " +
                             "compiled `.rs` file."),
        "targets_and_headers": ("A depset of strings, each one representing mapping of target to " +
                                "its headers in json format."),
    },
)

RustToolchainHeadersInfo = provider(
    doc = "A provider that contains all toolchain C++ headers",
    fields = {"headers": "depset"},
)

GeneratedBindingsInfo = provider(
    doc = "A provider that contains the generated C++ and Rust source files.",
    fields = {
        "cc_file": "The generated C++ source file.",
        "rust_file": "The generated Rust source file.",
    },
)

def _compile_cc(
        ctx,
        attr,
        cc_toolchain,
        feature_configuration,
        src,
        cc_infos):
    """Compiles a C++ source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      cc_toolchain: A cc_toolchain.
      feature_configuration: A feature configuration.
      src: The source file to be compiled.
      cc_infos: List[CcInfo]: A list of CcInfo dependencies.

    Returns:
      A CcInfo provider.
    """
    cc_info = cc_common.merge_cc_infos(cc_infos = cc_infos)

    (compilation_context, compilation_outputs) = cc_common.compile(
        name = src.basename,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        srcs = [src],
        grep_includes = ctx.file._grep_includes,
        user_compile_flags = attr.copts if hasattr(attr, "copts") else [],
        compilation_contexts = [cc_info.compilation_context],
    )

    (linking_context, _) = cc_common.create_linking_context_from_compilation_outputs(
        name = src.basename,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        compilation_outputs = compilation_outputs,
        linking_contexts = [cc_info.linking_context],
    )

    return CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
    )

def _get_crate_info(providers):
    for provider in providers:
        if hasattr(provider, "name"):
            return provider
    fail("Couldn't find a CrateInfo in the list of providers")

def _get_dep_info(providers):
    for provider in providers:
        if hasattr(provider, "direct_crates"):
            return provider
    fail("Couldn't find a DepInfo in the list of providers")

def _compile_rust(ctx, attr, src, deps):
    """Compiles a Rust source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      src: The source file to be compiled.
      deps: List[DepVariantInfo]: A list of dependencies needed.

    Returns:
      A DepVariantInfo provider.
    """
    toolchain = ctx.toolchains["@rules_rust//rust:toolchain"]

    output_hash = repr(hash(src.path))

    # TODO(b/216587072): Remove this hacky escaping and use the import! macro once available
    crate_name = ctx.label.name.replace("-", "_")

    lib_name = "{prefix}{name}-{lib_hash}{extension}".format(
        prefix = "lib",
        name = crate_name,
        lib_hash = output_hash,
        extension = ".rlib",
    )

    lib = ctx.actions.declare_file(lib_name)

    providers = rustc_compile_action(
        ctx = ctx,
        attr = attr,
        toolchain = toolchain,
        crate_info = rust_common.create_crate_info(
            name = crate_name,
            type = "rlib",
            root = src,
            srcs = depset([src]),
            deps = depset(deps),
            proc_macro_deps = depset([]),
            aliases = {},
            output = lib,
            edition = "2018",
            is_test = False,
            rustc_env = {},
            compile_data = depset([]),
            owner = ctx.label,
        ),
        output_hash = output_hash,
    )

    return DepVariantInfo(
        crate_info = _get_crate_info(providers),
        dep_info = _get_dep_info(providers),
        cc_info = None,
        build_info = None,
    )

def _generate_bindings(
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
                "--rs_out",
                rs_output.path,
                "--cc_out",
                cc_output.path,
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
        additional_outputs = [rs_output],
        variables = variables,
    )
    return (cc_output, rs_output)

def generate_and_compile_bindings(
        ctx,
        attr,
        compilation_context,
        public_hdrs,
        header_includes,
        action_inputs,
        targets_and_headers,
        deps_for_cc_file,
        deps_for_rs_file):
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
      deps_for_cc_file: list[CcInfo]: CcInfos needed by the generated C++ source file.
      deps_for_rs_file: list[DepVariantInfo]: DepVariantInfos needed by the generated Rust source file.

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

    cc_output, rs_output = _generate_bindings(
        ctx = ctx,
        attr = attr,
        cc_toolchain = cc_toolchain,
        feature_configuration = feature_configuration,
        compilation_context = compilation_context,
        public_hdrs = public_hdrs,
        header_includes = header_includes,
        action_inputs = action_inputs,
        targets_and_headers = targets_and_headers,
    )

    # Compile the "_rust_api_impl.cc" file
    cc_info = _compile_cc(
        ctx,
        attr,
        cc_toolchain,
        feature_configuration,
        cc_output,
        deps_for_cc_file,
    )

    # Compile the "_rust_api.rs" file
    dep_variant_info = _compile_rust(
        ctx,
        attr,
        rs_output,
        deps_for_rs_file,
    )

    return [
        RustBindingsFromCcInfo(
            cc_info = cc_info,
            dep_variant_info = dep_variant_info,
            targets_and_headers = targets_and_headers,
        ),
        GeneratedBindingsInfo(
            cc_file = cc_output,
            rust_file = rs_output,
        ),
        OutputGroupInfo(out = depset([cc_output, rs_output])),
    ]

def _get_hdrs_command_line(hdrs):
    return ["--public_headers=" + ",".join([x.path for x in hdrs])]

bindings_attrs = {
    "_cc_toolchain": attr.label(
        default = "@bazel_tools//tools/cpp:current_cc_toolchain",
    ),
    "_generator": attr.label(
        default = "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_target",
        executable = True,
        cfg = "exec",
    ),
    "_deps_for_bindings": attr.label(
        doc = "Dependencies that are needed to compile the generated .cc and .rs file.",
        default = "//rs_bindings_from_cc/bazel_support:deps_for_bindings",
    ),
    "_grep_includes": attr.label(
        allow_single_file = True,
        default = Label("@bazel_tools//tools/cpp:grep-includes"),
        cfg = "host",
    ),
    "_rustfmt": attr.label(
        default = "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_rustfmt_cfg": attr.label(
        default = "//nowhere:rustfmt.toml",
        allow_single_file = True,
    ),
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
}
