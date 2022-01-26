# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_binary.bzl",
    "GeneratedFilesDepsInfo",
)
load("//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")
load("//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")

# buildifier: disable=bzl-visibility
load("//third_party/bazel_rules/rules_rust/rust/private:common.bzl", "rust_common")

# buildifier: disable=bzl-visibility
load("//third_party/bazel_rules/rules_rust/rust/private:providers.bzl", "DepVariantInfo")

# buildifier: disable=bzl-visibility
load("//third_party/bazel_rules/rules_rust/rust/private:rustc.bzl", "rustc_compile_action")

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

GeneratedBindingsInfo = provider(
    doc = "A provider that contains the generated C++ and Rust source files.",
    fields = {
        "cc_file": "The generated C++ source file.",
        "rust_file": "The generated Rust source file.",
    },
)

# <internal link>/127#naming-header-files-h-and-inc recommends declaring textual headers either in the
# `textual_hdrs` attribute of the Blaze C++ rules, or using the `.inc` file extension. Therefore
# we are omitting ["inc"] from the list below.
_hdr_extensions = ["h", "hh", "hpp", "ipp", "hxx", "h++", "inl", "tlh", "tli", "H", "tcc"]

def _filter_none(input_list):
    return [element for element in input_list if element != None]

def _is_hdr(input):
    return input.path.split(".")[-1] in _hdr_extensions

def _filter_hdrs(input_list):
    return [hdr for hdr in input_list if _is_hdr(hdr)]

def _compile_cc(
        ctx,
        cc_toolchain,
        feature_configuration,
        target_cc_info,
        src,
        extra_deps):
    cc_info = cc_common.merge_cc_infos(
        cc_infos = [target_cc_info] + [
            dep[RustBindingsFromCcInfo].cc_info
            for dep in ctx.rule.attr.deps
            if RustBindingsFromCcInfo in dep
        ] + extra_deps,
    )

    (compilation_context, compilation_outputs) = cc_common.compile(
        name = src.basename,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        srcs = [src],
        grep_includes = ctx.file._grep_includes,
        user_compile_flags = ctx.rule.attr.copts if hasattr(ctx.rule.attr, "copts") else [],
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

def _compile_rust(ctx, src, extra_deps):
    deps = [
        dep[RustBindingsFromCcInfo].dep_variant_info
        for dep in ctx.rule.attr.deps
        if RustBindingsFromCcInfo in dep
    ] + extra_deps

    toolchain = ctx.toolchains["//third_party/bazel_rules/rules_rust/rust:toolchain"]

    output_hash = repr(hash(src.path))
    crate_name = ctx.label.name

    lib_name = "{prefix}{name}-{lib_hash}{extension}".format(
        prefix = "lib",
        name = crate_name,
        lib_hash = output_hash,
        extension = ".rlib",
    )

    lib = ctx.actions.declare_file(lib_name)

    providers = rustc_compile_action(
        ctx = ctx,
        attr = ctx.rule.attr,
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
        crate_info = providers[0],
        dep_info = providers[1],
        cc_info = None,
        build_info = None,
    )

public_headers_to_remove = {
    "//base:base": [
        "base/callback.h",  # //base:callback
        "base/callback-specializations.h",  # //base:callback
        "base/callback-types.h",  # //base:callback
        "base/file_toc.h",  # //base:file_toc
        "base/googleinit.h",  # //base:googleinit
        "base/logging.h",  # //base:logging
    ],
}

def _collect_hdrs(ctx):
    public_hdrs = _filter_hdrs(ctx.rule.files.hdrs)
    private_hdrs = _filter_hdrs(ctx.rule.files.srcs) if hasattr(ctx.rule.attr, "srcs") else []
    label = str(ctx.label)
    public_hdrs = [
        h
        for h in public_hdrs
        if h.short_path not in public_headers_to_remove.get(label, [])
    ]

    all_standalone_hdrs = public_hdrs + private_hdrs
    return public_hdrs, all_standalone_hdrs

def _rust_bindings_from_cc_aspect_impl(target, ctx):
    # We use a fake generator only when we are building the real one, in order to avoid
    # dependency cycles.
    if ctx.executable._generator.basename == "fake_rust_bindings_from_cc":
        return []

    # This is not a C++ rule
    if CcInfo not in target:
        return []

    if not hasattr(ctx.rule.attr, "hdrs"):
        return []

    public_hdrs, all_standalone_hdrs = _collect_hdrs(ctx)

    # At execution time we convert this depset to a json array that gets passed to our tool through
    # the --targets_and_headers flag.
    # We can improve upon this solution if:
    # 1. we use a library for parsing command line flags that allows repeated flags.
    # 2. instead of json string, we use a struct that will be expanded to flags at execution time.
    #    This requires changes to Blaze.
    targets_and_headers = depset(
        direct = [
            json.encode({
                "t": str(ctx.label),
                "h": [h.path for h in all_standalone_hdrs],
            }),
        ] if all_standalone_hdrs else [],
        transitive = [
            t[RustBindingsFromCcInfo].targets_and_headers
            for t in ctx.rule.attr.deps
            if RustBindingsFromCcInfo in t
        ],
    )

    if not public_hdrs:
        empty_cc_info = CcInfo()
        return RustBindingsFromCcInfo(
            cc_info = empty_cc_info,
            dep_variant_info = DepVariantInfo(cc_info = empty_cc_info),
            targets_and_headers = targets_and_headers,
        )

    hdrs_command_line = []
    if public_hdrs:
        hdrs_command_line.append("--public_headers=" + (",".join([x.short_path for x in public_hdrs])))

    header_includes = []
    for hdr in public_hdrs:
        header_includes.append("-include")
        header_includes.append(hdr.short_path)

    cc_toolchain = find_cpp_toolchain(ctx)

    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
        requested_features = ctx.features,
        unsupported_features = ctx.disabled_features + ["module_maps"],
    )

    cc_output = ctx.actions.declare_file(ctx.label.name + "_rust_api_impl.cc")
    rs_output = ctx.actions.declare_file(ctx.label.name + "_rust_api.rs")

    stl = ctx.attr._stl[CcInfo].compilation_context
    compilation_context = target[CcInfo].compilation_context
    variables = cc_common.create_compile_variables(
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        user_compile_flags = ctx.fragments.cpp.copts +
                             ctx.fragments.cpp.cxxopts +
                             header_includes + (
            ctx.rule.attr.copts if hasattr(ctx.rule.attr, "copts") else []
        ),
        preprocessor_defines = compilation_context.defines,
        system_include_directories = depset(
            cc_toolchain.built_in_include_directories,
            transitive = [stl.system_includes, compilation_context.system_includes],
        ),
        include_directories = depset(transitive = [stl.includes, compilation_context.includes]),
        quote_include_directories = depset(
            transitive = [stl.quote_includes, compilation_context.quote_includes],
        ),
        variables_extension = {
            "rs_bindings_from_cc_tool": ctx.executable._generator.path,
            "rs_bindings_from_cc_flags": [
                "--rs_out",
                rs_output.path,
                "--cc_out",
                cc_output.path,
            ] + hdrs_command_line,
            "targets_and_headers": targets_and_headers,
        },
    )

    # Run the `rs_bindings_from_cc` to generate the _rust_api_impl.cc and _rust_api.rs files.
    cc_common.create_compile_action(
        actions = ctx.actions,
        action_name = ACTION_NAMES.rs_bindings_from_cc,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        source_file = public_hdrs[0],
        output_file = cc_output,
        grep_includes = ctx.file._grep_includes,
        additional_inputs = depset(
            public_hdrs + [ctx.executable._rustfmt, ctx.executable._generator] + ctx.files._rustfmt_cfg,
        ),
        additional_outputs = [rs_output],
        variables = variables,
        compilation_context = compilation_context,
    )

    # Compile the "_rust_api_impl.cc" file
    cc_info = _compile_cc(
        ctx,
        cc_toolchain,
        feature_configuration,
        target[CcInfo],
        cc_output,
        ctx.attr._generator[GeneratedFilesDepsInfo].deps_for_cc_file,
    )

    # Compile the "_rust_api.rs" file
    dep_variant_info = _compile_rust(
        ctx,
        rs_output,
        ctx.attr._generator[GeneratedFilesDepsInfo].deps_for_rs_file,
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
    ]

rust_bindings_from_cc_aspect = aspect(
    implementation = _rust_bindings_from_cc_aspect_impl,
    attr_aspects = ["deps"],
    attrs = {
        "_cc_toolchain": attr.label(
            default = "//tools/cpp:current_cc_toolchain",
        ),
        "_generator": attr.label(
            default = "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_target",
            executable = True,
            cfg = "exec",
        ),
        "_grep_includes": attr.label(
            allow_single_file = True,
            default = Label("//tools/cpp:grep-includes"),
            cfg = "host",
        ),
        "_stl": attr.label(default = "//third_party/stl"),
        "_rustfmt": attr.label(
            default = "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_rustfmt_cfg": attr.label(
            default = "@rustfmt//:rustfmt.toml",
            allow_single_file = True,
        ),
        "_error_format": attr.label(
            default = "//third_party/bazel_rules/rules_rust:error_format",
        ),
        "_extra_rustc_flags": attr.label(
            default = "//third_party/bazel_rules/rules_rust:extra_rustc_flags",
        ),
        "_process_wrapper": attr.label(
            default = "//third_party/bazel_rules/rules_rust/util/process_wrapper",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
    },
    toolchains = [
        "//third_party/bazel_rules/rules_rust/rust:toolchain",
        "//tools/cpp:toolchain_type",
    ],
    host_fragments = ["cpp"],
    fragments = ["cpp", "google_cpp"],
)
