# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cc_bindings_from_rust` rule."""

load(
    "@rules_rust//rust:rust_common.bzl",
    "BuildInfo",
    "CrateInfo",
)

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:providers.bzl",
    "DepInfo",
    "DepVariantInfo",
)

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:rustc.bzl",
    "ExtraRustcFlagsInfo",
    "collect_deps",
    "collect_inputs",
    "construct_arguments",
)

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:utils.bzl",
    "find_toolchain",
)
load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_cli_flag_aspect_hint.bzl",
    "collect_cc_bindings_from_rust_cli_flags",
)
load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    "CcBindingsFromRustInfo",
    "GeneratedBindingsInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:compile_rust.bzl",
    "compile_rust",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain", "use_cpp_toolchain")

# Targets which do not receive C++ bindings at all.
targets_to_remove = [
]

def _get_dep_bindings_infos(ctx):
    """Returns `CcBindingsFromRustInfo`s of direct, non-transitive dependencies.

    Only information about direct, non-transitive dependencies is needed,
    because bindings for the public APIs may need to refer to types from
    such dependencies (e.g. `fn foo(param: TypeFromDirectDependency)`),
    but they cannot refer to types from transitive dependencies.

    Args:
      ctx: The rule context.

    Returns:
      A list of `CcBindingsFromRustInfo`s of all the direct, non-transitive Rust
      dependencies (dependencies of the Rust crate being used as input for
      `cc_bindings_from_rs`).
    """
    return [
        dep[CcBindingsFromRustInfo]
        for dep in ctx.rule.attr.deps + getattr(ctx.rule.attr, "cc_deps", [])
        if CcBindingsFromRustInfo in dep
    ]

def _generate_bindings(ctx, target, basename, inputs, args, rustc_env):
    """Invokes the `cc_bindings_from_rs` tool to generate C++ bindings for a Rust crate.

    Args:
      ctx: The rule context.
      target: The target crate.
      basename: The basename for the generated files
      inputs: `cc_bindings_from_rs` inputs specific to the target `crate`
      args: `rustc` and `process_wrapper` arguments from construct_arguments.
      rustc_env: `rustc` environment to use when running `cc_bindings_from_rs`

    Returns:
      The GeneratedBindingsInfo provider.
    """
    h_out_file = ctx.actions.declare_file(basename + "_cc_api.h")
    rs_out_file = ctx.actions.declare_file(basename + "_cc_api_impl.rs")

    crubit_args = ctx.actions.args()
    crubit_args.add("--h-out", h_out_file)
    crubit_args.add("--rs-out", rs_out_file)

    crubit_args.add("--crubit-support-path-format", "\"support/{header}\"")

    crubit_args.add("--clang-format-exe-path", ctx.file._clang_format)
    crubit_args.add("--rustfmt-exe-path", ctx.file._rustfmt)
    crubit_args.add("--rustfmt-config-path", ctx.file._rustfmt_cfg)

    for dep_bindings_info in _get_dep_bindings_infos(ctx):
        for header in dep_bindings_info.headers:
            arg = dep_bindings_info.crate_key + "=" + header.short_path
            crubit_args.add("--bindings-from-dependency", arg)

    outputs = [h_out_file, rs_out_file]
    if ctx.attr._generate_error_report[BuildSettingInfo].value:
        error_report_output = ctx.actions.declare_file(basename + "_cc_api_error_report.json")
        crubit_args.add(
            "--error-report-out",
            error_report_output.path,
        )
        outputs.append(error_report_output)

    for flag in collect_cc_bindings_from_rust_cli_flags(target, ctx):
        crubit_args.add(flag)
    ctx.actions.run(
        outputs = outputs,
        inputs = depset(
            [ctx.file._clang_format, ctx.file._rustfmt, ctx.file._rustfmt_cfg],
            transitive = [inputs],
        ),
        env = rustc_env,
        tools = [ctx.executable._cc_bindings_from_rs_tool],
        executable = ctx.executable._process_wrapper,
        mnemonic = "CcBindingsFromRust",
        progress_message = "Generating C++ bindings from Rust: %s" % h_out_file,
        # We don't use `args.all` here, because we want to do a couple of things:
        #
        # 1. specifically separate the crubit_args from the rustc_args, via `--`, putting crubit
        #    args first.
        # 2. change the rustc path to instead point to crubit.
        #
        # That said, if we passed arguments to crubit via environment variables or via flags that
        # can be interleaved with rustc flags in any order, and if we used _cc_bindings_from_rs_tool
        # as the tool_path for construct_arguments, then this could be `args.all` instead.

        # TODO(b/254049425): Remove `-Cpanic=abort` after crosstool contains cl/657372371.
        arguments = [args.process_wrapper_flags, "--", ctx.executable._cc_bindings_from_rs_tool.path, crubit_args, "--", args.rustc_flags, "-Cpanic=abort"],
    )

    return GeneratedBindingsInfo(
        h_file = h_out_file,
        rust_file = rs_out_file,
    )

def _make_cc_info_for_h_out_file(ctx, h_out_file, cc_infos):
    """Creates and returns CcInfo for the generated ..._cc_api.h header file.

    Args:
      ctx: The rule context.
      h_out_file: The generated "..._cc_api.h" header file
      cc_infos: cc_infos for dependencies of the h_out_file - should include both:
          1) the target `crate` and
          2) the compiled Rust glue crate (`..._cc_api_impl.rs` file).

    Returns:
      A CcInfo provider.
    """
    cc_info = cc_common.merge_cc_infos(cc_infos = [
        dep[CcInfo]
        for dep in ctx.attr._cc_deps_for_bindings
    ] + cc_infos)
    cc_toolchain = find_cpp_toolchain(ctx)
    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
    )
    (compilation_context, compilation_outputs) = cc_common.compile(
        name = ctx.label.name,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        public_hdrs = [h_out_file],
        compilation_contexts = [cc_info.compilation_context],
    )
    (linking_context, _) = cc_common.create_linking_context_from_compilation_outputs(
        name = ctx.label.name,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        compilation_outputs = compilation_outputs,
        linking_contexts = [cc_info.linking_context],
    )
    debug_context = cc_common.merge_debug_context([
        cc_info.debug_context(),
        cc_common.create_debug_context(compilation_outputs),
    ])
    return CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
        debug_context = debug_context,
    )

def _compile_rs_out_file(ctx, rs_out_file, target):
    """Compiles the generated "..._cc_api_impl.rs" file.

    Args:
      ctx: The rule context.
      rs_out_file: The generated "..._cc_api_impl.rs" file
      target: The target crate, e.g. as provided to `ctx.attr.crate`.

    Returns:
      CcInfo for the generated "..._cc_api_impl.rs".
    """
    deps = [
        DepVariantInfo(
            crate_info = dep[CrateInfo],
            dep_info = dep[DepInfo],
            cc_info = dep[CcInfo],
            build_info = None,
        )
        for dep in ctx.attr._rs_deps_for_bindings + [target]
    ]

    # The `..._cc_api_impl.rs` file needs to depend on all the deps of the target crate.
    deps += target[CrateInfo].deps.to_list()
    dep_variant_info = compile_rust(
        ctx,
        attr = ctx.rule.attr,
        src = rs_out_file,
        extra_srcs = [],
        deps = deps,
        crate_name = target[CrateInfo].name + "_cc_api_impl",
        include_coverage = True,
        force_all_deps_direct = False,
    )
    return dep_variant_info.cc_info

def _cc_bindings_from_rust_aspect_impl(target, ctx):
    basename = target.label.name

    if CrateInfo not in target:
        return []
    if str(target.label) in targets_to_remove:
        return []

    toolchain = find_toolchain(ctx)
    crate_info = target[CrateInfo]
    cc_toolchain = find_cpp_toolchain(ctx)
    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
    )

    dep_info, build_info, linkstamps = collect_deps(
        deps = crate_info.deps,
        proc_macro_deps = crate_info.proc_macro_deps,
        aliases = crate_info.aliases,
    )

    compile_inputs, out_dir, build_env_files, build_flags_files, linkstamp_outs, ambiguous_libs = collect_inputs(
        ctx = ctx,
        file = ctx.file,
        files = ctx.files,
        linkstamps = linkstamps,
        toolchain = toolchain,
        cc_toolchain = cc_toolchain,
        feature_configuration = feature_configuration,
        crate_info = crate_info,
        dep_info = dep_info,
        build_info = build_info,
        stamp = False,
        experimental_use_cc_common_link = False,
    )

    # TODO(b/282958841): The `collect_inputs` call above should take the `data`
    # dependency into account.
    data_files = [target.files for target in getattr(ctx.rule.attr, "data", [])]
    compile_inputs = depset(transitive = [compile_inputs] + data_files)

    args, env = construct_arguments(
        ctx = ctx,
        attr = ctx.rule.attr,
        file = ctx.file,
        toolchain = toolchain,
        tool_path = toolchain.rustc.path,
        cc_toolchain = cc_toolchain,
        emit = [],
        feature_configuration = feature_configuration,
        crate_info = crate_info,
        dep_info = dep_info,
        linkstamp_outs = linkstamp_outs,
        ambiguous_libs = ambiguous_libs,
        # TODO(lukasza): Do we need to pass an output_hash here?
        # b/254690602 suggests that we want to include a hash in
        # the names of namespaces generated by cc_bindings_from_rs.
        output_hash = "",
        rust_flags = ctx.attr._extra_rustc_flags[ExtraRustcFlagsInfo].extra_rustc_flags +
                     ctx.attr._extra_rustc_flag[ExtraRustcFlagsInfo].extra_rustc_flags,
        out_dir = out_dir,
        build_env_files = build_env_files,
        build_flags_files = build_flags_files,
        force_all_deps_direct = False,
        stamp = False,
        use_json_output = False,
        skip_expanding_rustc_env = True,
    )

    bindings_info = _generate_bindings(
        ctx,
        target,
        basename,
        compile_inputs,
        args,
        env,
    )

    impl_cc_info = _compile_rs_out_file(ctx, bindings_info.rust_file, target)

    cc_info = _make_cc_info_for_h_out_file(
        ctx,
        bindings_info.h_file,
        cc_infos = [target[CcInfo], impl_cc_info] + [
            dep_bindings_info.cc_info
            for dep_bindings_info in _get_dep_bindings_infos(ctx)
        ],
    )
    return [
        CcBindingsFromRustInfo(
            cc_info = cc_info,
            crate_key = crate_info.name,
            headers = [bindings_info.h_file],
        ),
        bindings_info,
        OutputGroupInfo(out = depset([bindings_info.h_file, bindings_info.rust_file])),
    ]

cc_bindings_from_rust_aspect = aspect(
    implementation = _cc_bindings_from_rust_aspect_impl,
    doc = "Aspect for generating C++ bindings for a Rust library.",
    attr_aspects = ["deps"],
    attrs = {
        "_cc_bindings_from_rs_tool": attr.label(
            default = Label("//cc_bindings_from_rs:cc_bindings_from_rs"),
            executable = True,
            cfg = "exec",
            allow_single_file = True,
        ),
        "_cc_toolchain": attr.label(
            default = "@bazel_tools//tools/cpp:current_cc_toolchain",
        ),
        "_clang_format": attr.label(
            default = "//third_party/crosstool/google3_users:stable_clang-format",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_cc_deps_for_bindings": attr.label_list(
            doc = "Dependencies needed to build the C++ sources generated by cc_bindings_from_rs.",
            default = [
                "//support/internal:bindings_support",
                "//support/rs_std:rs_char",
            ],
        ),
        "_process_wrapper": attr.label(
            default = "@rules_rust//util/process_wrapper",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_rs_deps_for_bindings": attr.label_list(
            doc = "Dependencies needed to build the Rust sources generated by cc_bindings_from_rs.",
            default = [],
        ),
        "_rustfmt": attr.label(
            default = "//third_party/crosstool/rust/unstable:genrustfmt_for_crubit_aspects",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_rustfmt_cfg": attr.label(
            default = "//nowhere:rustfmt.toml",
            allow_single_file = True,
        ),
        "_extra_rustc_flags": attr.label(
            default = Label("@rules_rust//:extra_rustc_flags"),
        ),
        "_extra_rustc_flag": attr.label(
            default = Label("@rules_rust//:extra_rustc_flag"),
        ),
        "_generate_error_report": attr.label(
            default = "//cc_bindings_from_rs/bazel_support:generate_error_report",
        ),
    },
    toolchains = [
        "@rules_rust//rust:toolchain_type",
    ] + use_cpp_toolchain(),
    fragments = ["cpp"],
)

def _cc_bindings_from_rust_rule_impl(ctx):
    crate = ctx.attr.crate
    return [
        crate[CcBindingsFromRustInfo].cc_info,
        # If we try to generate rust bindings of c++ bindings of this rust crate, we get back
        # the original rust crate again.
        RustBindingsFromCcInfo(
            cc_info = None,
            dep_variant_info = DepVariantInfo(
                crate_info = crate[CrateInfo] if CrateInfo in crate else None,
                dep_info = crate[DepInfo] if DepInfo in crate else None,
                build_info = crate[BuildInfo] if BuildInfo in crate else None,
                cc_info = crate[CcInfo] if CcInfo in crate else None,
            ),
            target_args = depset([]),
            namespaces = None,
        ),
    ]

cc_bindings_from_rust = rule(
    implementation = _cc_bindings_from_rust_rule_impl,
    doc = "Rule for generating C++ bindings for a Rust library.",
    attrs = {
        "crate": attr.label(
            doc = "Rust library to generate C++ bindings for",
            allow_files = False,
            mandatory = True,
            providers = [CcBindingsFromRustInfo],
            aspects = [cc_bindings_from_rust_aspect],
        ),
    },
)
