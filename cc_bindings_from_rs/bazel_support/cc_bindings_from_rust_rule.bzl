# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cc_bindings_from_rust` rule."""

load("@rules_cc//cc/common:cc_common.bzl", "cc_common")
load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
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
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl",
    "crate_name_to_library_config",
    "get_additional_cc_hdrs_and_srcs",
)
load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    "CcBindingsFromRustInfo",
    "GeneratedBindingsInfo",
)
load(
    "//common/bazel_support:cc_info_util.bzl",
    "get_static_libraries_from_cc_info",
)
load(
    "//features:crubit_feature_hint.bzl",
    "find_crubit_features",
)
load(
    "//features:global_features.bzl",
    "SUPPORTED_FEATURES",
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

def _target_name_to_include_guard(target):
    return "".join([
        c if c.isalnum() else "_"
        for c in (target.label.package + "/" + target.label.name).upper().elems()
    ])

def _generate_bindings(ctx, target, basename, inputs, args, rustc_env, proto_crate_renames):
    """Invokes the `cc_bindings_from_rs` tool to generate C++ bindings for a Rust crate.

    Args:
      ctx: The rule context.
      target: The target crate.
      basename: The basename for the generated files
      inputs: `cc_bindings_from_rs` inputs specific to the target `crate`
      args: `rustc` and `process_wrapper` arguments from construct_arguments.
      rustc_env: `rustc` environment to use when running `cc_bindings_from_rs`
      proto_crate_renames: Mapping of the `rust_proto_library` to the `proto_library` crate name.

    Returns:
      A tuple of (GeneratedBindingsInfo, features, current_config, output_depset).
    """
    h_out_file = ctx.actions.declare_file(basename + ".h")
    rs_out_file = ctx.actions.declare_file(basename + "_cc_api_impl.rs")

    if ctx.label in [Label(x) for x in ctx.attr._verbose_log_targets[BuildSettingInfo].value]:
        verbose_log_env = {"RUST_LOG": "info"}
    else:
        verbose_log_env = {"RUST_LOG": "error"}

    crubit_args = ctx.actions.args()
    crubit_args.add("--h-out", h_out_file)
    crubit_args.add("--rs-out", rs_out_file)
    crubit_args.add("--h-out-include-guard", _target_name_to_include_guard(target))

    crubit_args.add("--crubit-support-path-format", "\"support/{header}\"")

    crubit_args.add("--clang-format-exe-path", ctx.file._clang_format)
    crubit_args.add("--rustfmt-exe-path", ctx.file._rustfmt)
    crubit_args.add("--rustfmt-config-path", ctx.file._rustfmt_cfg)

    for dep_bindings_info in _get_dep_bindings_infos(ctx):
        for header in dep_bindings_info.headers:
            arg = dep_bindings_info.crate_key + "=" + header.short_path
            crubit_args.add("--bindings-from-dependency", arg)
        for feature in dep_bindings_info.features:
            arg = dep_bindings_info.crate_key + "=" + feature
            crubit_args.add("--crate-feature", arg)

    crubit_args.add("--default-features", ",".join(SUPPORTED_FEATURES))

    features = find_crubit_features(target, ctx)

    for feature in features:
        crubit_args.add("--crate-feature", "self=" + feature)

    outputs = [h_out_file, rs_out_file]
    if ctx.attr._generate_error_report[BuildSettingInfo].value:
        error_report_output = ctx.actions.declare_file(basename + "_cc_api_error_report.json")
        crubit_args.add(
            "--error-report-out",
            error_report_output.path,
        )
        outputs.append(error_report_output)
    config = crate_name_to_library_config(ctx)
    current_config = config.get("self", None)
    for crate_name, crate_config in config.items():
        if crate_config.namespace:
            crubit_args.add("--crate-namespace", crate_name + "=" + crate_config.namespace)
    for mapping in proto_crate_renames:
        crubit_args.add("--crate-rename", mapping.crate_name + "=" + mapping.old_crate_name)
    for flag in collect_cc_bindings_from_rust_cli_flags(target, ctx):
        crubit_args.add(flag)
    toolchain = ctx.toolchains["//cc_bindings_from_rs/bazel_support:toolchain_type"]
    if toolchain == None:
        ctx.actions.run_shell(
            command = (
                "echo 'Crubit (cc_bindings_from_rs) is not available on this platform\n" +
                "To debug, rerun with --toolchain_resolution_debug=//cc_bindings_from_rs/bazel_support:toolchain_type'" +
                " && false"
            ),
            outputs = outputs,
            mnemonic = "CcBindingsFromRustUnsupported",
        )
    else:
        toolchain = toolchain.cc_bindings_from_rs_toolchain_info
        ctx.actions.run(
            outputs = outputs,
            inputs = depset(
                [ctx.file._clang_format, ctx.file._rustfmt, ctx.file._rustfmt_cfg],
                transitive = [inputs],
            ),
            env = rustc_env | verbose_log_env,
            tools = [toolchain.binary],
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
            # can be interleaved with rustc flags in any order, and if we used toolchain.binary
            # as the tool_path for construct_arguments, then this could be `args.all` instead.
            arguments = [args.process_wrapper_flags, "--", toolchain.binary.path, crubit_args, "--", args.rustc_flags],
            toolchain = "//cc_bindings_from_rs/bazel_support:toolchain_type",
        )

    generated_bindings_info = GeneratedBindingsInfo(
        h_file = h_out_file,
        rust_file = rs_out_file,
    )
    output_depset = [x for x in outputs if x != None]

    return generated_bindings_info, features, current_config, output_depset

def _make_cc_info_for_h_out_file(ctx, h_out_file, extra_cc_hdrs, extra_cc_srcs, cc_infos):
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
        srcs = extra_cc_srcs,
        public_hdrs = [h_out_file] + extra_cc_hdrs,
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
        cc_info._debug_context if hasattr(cc_info, "_debug_context") else cc_info.debug_context(),
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
      DepVariantInfo for the generated "..._cc_api_impl.rs".
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
    return compile_rust(
        ctx,
        attr = ctx.rule.attr,
        src = rs_out_file,
        extra_srcs = [],
        deps = depset(deps),
        crate_name = target[CrateInfo].name + "_cc_api_impl",
        include_coverage = True,
        force_all_deps_direct = False,
    )

def _cc_bindings_from_rust_aspect_impl(target, ctx):
    basename = target.label.name

    if CrateInfo not in target:
        return []
    if str(target.label) in targets_to_remove:
        return []

    proto_crate_renames = []

    toolchain = find_toolchain(ctx)
    crate_info = target[CrateInfo]
    cc_toolchain = find_cpp_toolchain(ctx)
    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
    )

    dep_info, build_info, linkstamps = collect_deps(
        deps = crate_info.deps.to_list(),
        proc_macro_deps = crate_info.proc_macro_deps.to_list(),
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
        lint_files = [],
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

    bindings_info, features, config, output_depset = _generate_bindings(
        ctx,
        target,
        basename,
        compile_inputs,
        args,
        env,
        proto_crate_renames,
    )

    dep_variant_info = _compile_rs_out_file(ctx, bindings_info.rust_file, target)

    (extra_cc_hdrs, extra_cc_srcs) = get_additional_cc_hdrs_and_srcs(ctx)

    cc_info = _make_cc_info_for_h_out_file(
        ctx,
        bindings_info.h_file,
        extra_cc_hdrs,
        extra_cc_srcs,
        cc_infos = [target[CcInfo], dep_variant_info.cc_info] + [
            dep_bindings_info.cc_info
            for dep_bindings_info in _get_dep_bindings_infos(ctx)
        ],
    )

    out_compiled = depset(
        [dep_variant_info.crate_info.output] +
        get_static_libraries_from_cc_info(cc_info),
    )

    return [
        CcBindingsFromRustInfo(
            cc_info = cc_info,
            crate_key = crate_info.name,
            headers = [bindings_info.h_file],
            features = features,
            configuration = config,
        ),
        bindings_info,
        OutputGroupInfo(out = output_depset, out_compiled = out_compiled),
    ]

cc_bindings_from_rust_aspect = aspect(
    implementation = _cc_bindings_from_rust_aspect_impl,
    doc = "Aspect for generating C++ bindings for a Rust library.",
    attr_aspects = ["deps"],
    attrs = {
        "_clang_format": attr.label(
            default = Label("@llvm_toolchain//:clang-format"),
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_cc_deps_for_bindings": attr.label_list(
            doc = "Dependencies needed to build the C++ sources generated by cc_bindings_from_rs.",
            default = [
                "//support/public:bindings_support",
                "//support:annotations_internal",
                "//support/public:char",
                "//support/public:slice_ref",
                "//support/public:str_ref",
                "//support:bridge_cpp",
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
            default = [
                "//support:bridge_rust",
            ],
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
        "_globally_enabled_features": attr.label(
            default = "//common/bazel_support:globally_enabled_features",
        ),
        "_verbose_log_targets": attr.label(
            default = "//common/bazel_support:verbose_log_targets",
        ),
    },
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        config_common.toolchain_type("//cc_bindings_from_rs/bazel_support:toolchain_type", mandatory = False),
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
            pass_through_dep_variant_infos = depset(),
            target_args = depset([
                # Mark the headers as being owned by the (bindings for the) crate, so that
                # rs_bindings_from_cc doesn't assume that they're owned by the current target or
                # that they don't have bindings.
                json.encode(
                    {
                        "t": str(crate.label),
                        "h": [h.path for h in crate[CcBindingsFromRustInfo].headers],
                        # Note: the feature set is a lie, crubit won't run on this target.
                        # The "pun" we're relying on is that Crubit will think that
                        # "everything will get bindings if I run rs_bindings_from_cc on it".
                        # In fact, we _won't_ run rs_bindings_from_cc on it, but everything "will
                        # get bindings" because it was originally defined in Rust to begin with.
                        # TODO(jeanpierreda): Maybe define an assume_bindings/round_trip feature?
                        "f": ["all"],
                    },
                ),
            ]),
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
