# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cc_bindings_from_rust` rule."""

load("@rules_cc//cc/common:cc_common.bzl", "cc_common")
load("@rules_cc//cc/common:cc_helper.bzl", "cc_helper")
load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
load("@rules_rust//rust:defs.bzl", "rust_common")
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
    "CcBindingsFromRustCliFlagInfo",
    "collect_cc_bindings_from_rust_cli_flags",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl",
    "CcBindingsFromRustLibraryConfigInfo",
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
    "get_static_library_files",
)
load(
    "//features:crubit_feature_hint.bzl",
    "CrubitFeaturesInfo",
    "find_crubit_features",
    "find_denied_crubit_features",
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

def _get_dep_bindings_infos(attr):
    """Returns `CcBindingsFromRustInfo`s of direct, non-transitive dependencies.

    Only information about direct, non-transitive dependencies is needed,
    because bindings for the public APIs may need to refer to types from
    such dependencies (e.g. `fn foo(param: TypeFromDirectDependency)`),
    but they cannot refer to types from transitive dependencies.

    Args:
      attr: The rule attributes.

    Returns:
      A list of `CcBindingsFromRustInfo`s of all the direct, non-transitive Rust
      dependencies (dependencies of the Rust crate being used as input for
      `cc_bindings_from_rs`).
    """
    return [
        dep[CcBindingsFromRustInfo]
        for dep in getattr(attr, "deps", []) + getattr(attr, "cc_deps", [])
        if CcBindingsFromRustInfo in dep
    ]

def _target_name_to_include_guard(label):
    return "".join([
        c if c.isalnum() else "_"
        for c in (label.package + "/" + label.name).upper().elems()
    ])

def _get_dirname(file):
    return file.dirname

def _add_rustc_lib_env(ctx, tool_args):
    """Configures `LD_LIBRARY_PATH` via process_wrapper `--subst` for `rustc_lib`.

    Needed in Bazel where `rustc_private` dynamically links against `librustc_driver`
    (statically linked internally). Passing the directory via `--subst` lets Bazel path
    mapping rewrite it instead of baking an output-root path into the action environment.

    Args:
      ctx: The rule context.
      tool_args: `Args` object for process_wrapper flags before `--`.

    Returns:
      A dictionary of environment variables to merge into the action environment.
    """
    rust_toolchain = ctx.toolchains["@rules_rust//rust:toolchain_type"]
    rustc_lib = rust_toolchain.rustc_lib.to_list() if rust_toolchain != None else []
    if not rustc_lib:
        return {}
    tool_args.add_all(
        [rustc_lib[0]],
        before_each = "--subst",
        format_each = "rustc_lib_dir=%s",
        map_each = _get_dirname,
        expand_directories = False,
    )
    return {"LD_LIBRARY_PATH": "${rustc_lib_dir}"}

def _get_crate_dirname(crate):
    """A helper macro for getting the directory name of the current crate's output path.

    This must be a function because `map_each` does not accept a lambda.

    Args:
        crate (CrateInfo|AliasableDepInfo): A CrateInfo or an AliasableDepInfo provider

    Returns:
        str: The directory name of the the output File that will be produced.
    """
    if hasattr(crate, "dep"):
        return crate.dep.output.dirname
    return crate.output.dirname

def _build_crubit_rustc_args(ctx, toolchain, original_args = None, dep_info = None):
    """Builds the subset of rustc arguments supported by `cc_bindings_from_rs`.

    Args:
      ctx: The rule context.
      toolchain: The Rust toolchain.
      original_args: Optional arguments from `construct_arguments` (for crate targets).
      dep_info: Optional `DepInfo` whose direct and transitive crates provide `-Ldependency=`.

    Returns:
      A struct with `process_wrapper_flags`, `rustc_flags`, and `supports_path_mapping`.
    """
    rustc_flags = ctx.actions.args()
    rustc_flags.add_all(
        toolchain.rust_std,
        before_each = "-L",
        map_each = _get_dirname,
        uniquify = True,
    )
    rustc_flags.add(toolchain.target_flag_value, format = "--target=%s")
    if dep_info != None:
        crates = depset(
            transitive = [
                dep_info.direct_crates,
                dep_info.transitive_crates,
            ],
        )
        rustc_flags.add_all(
            crates,
            map_each = _get_crate_dirname,
            uniquify = True,
            format_each = "-Ldependency=%s",
        )
    supports_path_mapping = (
        getattr(original_args, "supports_path_mapping", False) if original_args != None else True
    )
    if toolchain._toolchain_generated_sysroot:
        sysroot_anchor = getattr(toolchain, "sysroot_anchor", None)
        if sysroot_anchor != None:
            rustc_flags.add_all(
                [sysroot_anchor],
                map_each = _get_dirname,
                format_each = "--sysroot=%s",
            )
        else:
            rustc_flags.add(toolchain.sysroot, format = "--sysroot=%s")
            supports_path_mapping = False
    return struct(
        process_wrapper_flags = (
            original_args.process_wrapper_flags if original_args != None else ctx.actions.args()
        ),
        rustc_flags = rustc_flags,
        supports_path_mapping = supports_path_mapping,
    )

def _generate_bindings(ctx, dep_bindings_infos, config, label, features, cli_flags, crate_name, basename, inputs, args, rustc_env, proto_crate_renames, self_rmeta, is_golden_test_override = None, target_denied_features = [], ignore_symbols_from_files = []):
    """Invokes the `cc_bindings_from_rs` tool to generate C++ bindings for a Rust crate.

    Args:
      ctx: The rule context.
      dep_bindings_infos: `CcBindingsFromRustInfo`s of direct, non-transitive dependencies.
      config: Configuration for `cc_bindings_from_rs`.
      label: The label of the target crate.
      features: C++ feature configuration.
      cli_flags: Crubit CLI flags.
      crate_name: The name of the target crate.
      basename: The basename for the generated files
      inputs: `cc_bindings_from_rs` inputs specific to the target `crate`
      args: `rustc` and `process_wrapper` arguments from construct_arguments.
      rustc_env: `rustc` environment to use when running `cc_bindings_from_rs`
      proto_crate_renames: Mapping of the `rust_proto_library` to the `proto_library` crate name.
      self_rmeta: The rmeta file for the current crate.
      is_golden_test_override: Overrides value of `--is-golden-test` flag instead of checking for the transition.
      target_denied_features: Features denied for this target by `SUPPORTED_FEATURE_DENYLISTS`, overriding `--default-features`.
      ignore_symbols_from_files: List of Files whose symbols should be ignored (passed via `--ignore-symbols-from-files`).

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
    crubit_args.add("--h-out-include-guard", _target_name_to_include_guard(label))

    crubit_args.add("--crubit-support-path-format", "\"support/{header}\"")

    if ctx.executable._clang_format:
        crubit_args.add("--clang-format-exe-path", ctx.executable._clang_format)
    crubit_args.add("--rustfmt-exe-path", ctx.file._rustfmt)
    crubit_args.add("--rustfmt-config-path", ctx.file._rustfmt_cfg)

    for dep_bindings_info in dep_bindings_infos:
        for header in dep_bindings_info.headers:
            arg = dep_bindings_info.crate_key + "=" + header.short_path
            crubit_args.add("--crate-header", arg)
        for feature in dep_bindings_info.features:
            arg = dep_bindings_info.crate_key + "=" + feature
            crubit_args.add("--crate-feature", arg)

    toolchain = ctx.toolchains["//cc_bindings_from_rs/bazel_support:toolchain_type"]
    if toolchain != None:
        unsupported_features = getattr(toolchain.cc_bindings_from_rs_toolchain_info, "unsupported_features", [])
        default_features = [f for f in SUPPORTED_FEATURES if f not in unsupported_features]
        features = [f for f in features if f not in unsupported_features]
    else:
        unsupported_features = []
        default_features = SUPPORTED_FEATURES

    crubit_args.add("--default-features", ",".join(default_features))

    for feature in features:
        crubit_args.add("--crate-feature", "self=" + feature)

    # Features denied by `SUPPORTED_FEATURE_DENYLISTS` would otherwise come back via
    # `--default-features`.
    for dep_bindings_info in dep_bindings_infos:
        for feature in getattr(dep_bindings_info, "target_denied_features", []):
            if feature not in unsupported_features:
                crubit_args.add("--crate-disabled-feature", dep_bindings_info.crate_key + "=" + feature)
    for feature in target_denied_features:
        if feature not in unsupported_features:
            crubit_args.add("--crate-disabled-feature", "self=" + feature)

    outputs = [h_out_file, rs_out_file]
    if ctx.attr._generate_error_report[BuildSettingInfo].value:
        error_report_output = ctx.actions.declare_file(basename + "_cc_api_error_report.json")
        crubit_args.add(
            "--error-report-out",
            error_report_output,
        )
        outputs.append(error_report_output)
    current_config = config.get("self", None)
    for _crate_name, crate_config in config.items():
        if crate_config.namespace:
            crubit_args.add("--crate-namespace", _crate_name + "=" + crate_config.namespace)
    for mapping in proto_crate_renames:
        crubit_args.add("--crate-rename", mapping.crate_name + "=" + mapping.old_crate_name)
    self_crate_name = None
    for flag in cli_flags:
        # If --source-crate-name was specified manually, prefer that over passing the default.
        if flag.startswith("--source-crate-name="):
            self_crate_name = flag.removeprefix("--source-crate-name=")
        crubit_args.add(flag)
    if ignore_symbols_from_files:
        crubit_args.add_all(
            ignore_symbols_from_files,
            format_each = "--ignore-symbols-from-files=%s",
        )
    if self_crate_name == None:
        self_crate_name = crate_name
        crubit_args.add("--source-crate-name", self_crate_name)
        if self_rmeta != None:
            crubit_args.add(self_rmeta, format = "--extern=" + self_crate_name + "=%s")

    crubit_args.add("--enable-rmeta-interface")
    crubit_args.add("--portable-abi-compatible")
    is_golden_test = is_golden_test_override if is_golden_test_override != None else ctx.attr._is_golden_test[BuildSettingInfo].value
    if is_golden_test:
        crubit_args.add("--is-golden-test")
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
        tool_args = ctx.actions.args()
        action_env = rustc_env | verbose_log_env | _add_rustc_lib_env(ctx, tool_args)
        tool_args.add("--", toolchain.binary)
        out_dir_prefix = ctx.bin_dir.path.split("/")[0] + "/"
        supports_path_mapping = (
            args.supports_path_mapping and
            not any([
                out_dir_prefix in v
                for v in action_env.values()
            ])
        )
        ctx.actions.run(
            outputs = outputs,
            inputs = depset(
                [f for f in [ctx.file._rustfmt_cfg, self_rmeta] if f != None],
                transitive = [inputs],
            ),
            env = action_env,
            tools = [toolchain.binary, ctx.executable._rustfmt, ctx.executable._clang_format],
            executable = ctx.executable._process_wrapper,
            mnemonic = "CcBindingsFromRust",
            progress_message = "Generating C++ bindings from Rust: %s" % h_out_file,
            # We don't use `args.all` here, because we want to change the rustc path to instead
            # point to crubit.
            #
            # That said, if we passed arguments to crubit via environment variables or via flags that
            # can be interleaved with rustc flags in any order, and if we used toolchain.binary
            # as the tool_path for construct_arguments, then this could be `args.all` instead.
            arguments = [args.process_wrapper_flags, tool_args, crubit_args, args.rustc_flags],
            execution_requirements = {"supports-path-mapping": ""} if supports_path_mapping else None,
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
      A tuple `(cc_info, library_to_link)`: the `CcInfo` provider for the header and its
      dependencies, and the `LibraryToLink` holding only the result of compiling `extra_cc_srcs`
      (`None` if there is none).
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
    (linking_context, linking_outputs) = cc_common.create_linking_context_from_compilation_outputs(
        name = ctx.label.name,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        compilation_outputs = compilation_outputs,
        linking_contexts = [cc_info.linking_context],
    )

    debug_context = cc_helper.merge_cc_debug_contexts(compilation_outputs, [cc_info])

    cc_info = CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
        debug_context = debug_context,
    )
    return cc_info, linking_outputs.library_to_link

def _compile_rs_out_file(ctx, attr, rs_out_file, crate_name, deps):
    """Compiles the generated "..._cc_api_impl.rs" file.

    Args:
      ctx: The rule context.
      attr: The rule attributes.
      rs_out_file: The generated "..._cc_api_impl.rs" file
      crate_name: The name of the crate.
      deps: The dependencies of the target crate.

    Returns:
      DepVariantInfo for the generated "..._cc_api_impl.rs".
    """
    _deps = [
        DepVariantInfo(
            crate_info = dep[CrateInfo],
            dep_info = dep[DepInfo],
            cc_info = dep[CcInfo],
            build_info = None,
        )
        for dep in ctx.attr._rs_deps_for_bindings + deps
    ]

    # The `..._cc_api_impl.rs` file needs to depend on all the deps of the target crate.
    _deps += [dep for target in deps for dep in target[CrateInfo].deps.to_list()]
    return compile_rust(
        ctx,
        attr = attr,
        src = rs_out_file,
        extra_srcs = [],
        deps = depset(_deps),
        crate_name = crate_name + "_cc_api_impl",
        include_coverage = True,
    )

def _cc_bindings_from_rust_aspect_impl(target, ctx):
    basename = target.label.name

    if CrateInfo not in target:
        return []
    if CcBindingsFromRustInfo in target:
        return []
    if target.label in targets_to_remove:
        return []

    if target.label == Label("//support:cpp_std"):
        # `cpp_std` is a pure Rust re-export of `cc_std::std::*`. Its C++ bindings are the
        # C++ standard library headers from `cc_std` in namespace `::std`, rather than a
        # `cc_bindings_from_rs`-generated `cpp_std.h` header.
        cc_std_infos = _get_dep_bindings_infos(ctx.rule.attr)
        if len(cc_std_infos) != 1:
            fail("Expected //support:cpp_std to have 1 bindings dep, got: %s" % cc_std_infos)
        cc_std_info = cc_std_infos[0]
        return [
            CcBindingsFromRustInfo(
                cc_info = cc_std_info.cc_info,
                crate_key = target[CrateInfo].name,
                headers = cc_std_info.headers,
                features = cc_std_info.features,
                target_denied_features = getattr(cc_std_info, "target_denied_features", []),
                configuration = CcBindingsFromRustLibraryConfigInfo(
                    namespace = "std",
                    extra_cc_hdrs = [],
                    extra_cc_srcs = [],
                ),
                transitive_infos = depset(
                    direct = [cc_std_info],
                    transitive = [getattr(cc_std_info, "transitive_infos", depset())] if hasattr(cc_std_info, "transitive_infos") else [],
                ),
            ),
        ]

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

    original_args, env = construct_arguments(
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
        build_flags_files = depset(),
        force_all_deps_direct = False,
        stamp = False,
        use_json_output = False,
        skip_expanding_rustc_env = True,
    )
    args = _build_crubit_rustc_args(ctx, toolchain, original_args = original_args, dep_info = dep_info)

    if target[CrateInfo].metadata != None:
        rmeta = target[CrateInfo].metadata
    else:
        # Expectation is this will be an rlib file.
        rmeta = target[CrateInfo].output

    features = find_crubit_features(target, ctx)
    target_denied_features = find_denied_crubit_features(target, ctx)
    cli_flags = collect_cc_bindings_from_rust_cli_flags(target, ctx)

    ignore_symbols_from_files = [
        file
        for file in depset(transitive = [
            dep[RustBindingsFromCcInfo].additional_rust_srcs
            for dep in getattr(ctx.rule.attr, "deps", []) + getattr(ctx.rule.attr, "cc_deps", [])
            if RustBindingsFromCcInfo in dep
        ]).to_list()
        # These are internal to Crubit and we want them receive bindings despite being in additional
        # sources.
        if "cc_std_impl" not in file.path and "cc_std_impl" not in file.short_path
    ]
    stdlib_info = ctx.attr._stdlib_bindings[CcBindingsFromRustInfo]
    stdlib_bindings_infos = [stdlib_info] + stdlib_info.transitive_infos.to_list()
    dep_bindings_infos = _get_dep_bindings_infos(ctx.rule.attr) + stdlib_bindings_infos
    config = crate_name_to_library_config(
        aspect_hints = ctx.rule.attr.aspect_hints,
        rust_infos = dep_bindings_infos,
    )
    (extra_cc_hdrs, extra_cc_srcs) = get_additional_cc_hdrs_and_srcs(ctx.rule.attr.aspect_hints)
    for hdr in extra_cc_hdrs:
        if hdr.short_path.endswith("_extracted_cc.h"):
            cli_flags.append("--extra-rs-srcs-include=" + hdr.short_path)

    bindings_info, features, config, output_depset = _generate_bindings(
        ctx,
        dep_bindings_infos = dep_bindings_infos,
        config = config,
        label = target.label,
        features = features,
        cli_flags = cli_flags,
        crate_name = crate_info.name,
        basename = basename,
        inputs = compile_inputs,
        args = args,
        rustc_env = env,
        proto_crate_renames = proto_crate_renames,
        self_rmeta = rmeta,
        target_denied_features = target_denied_features,
        ignore_symbols_from_files = ignore_symbols_from_files,
    )

    dep_variant_info = _compile_rs_out_file(ctx, ctx.rule.attr, bindings_info.rust_file, target[CrateInfo].name, [target])

    cc_info, extra_cc_library_to_link = _make_cc_info_for_h_out_file(
        ctx,
        bindings_info.h_file,
        extra_cc_hdrs,
        extra_cc_srcs,
        cc_infos = [target[CcInfo], dep_variant_info.cc_info] + [
            dep_bindings_info.cc_info
            for dep_bindings_info in dep_bindings_infos
        ],
    )

    out_compiled = depset(
        [dep_variant_info.crate_info.output] +
        get_static_library_files(extra_cc_library_to_link),
    )

    return [
        CcBindingsFromRustInfo(
            cc_info = cc_info,
            crate_key = crate_info.name,
            headers = [bindings_info.h_file],
            features = features,
            target_denied_features = target_denied_features,
            configuration = config,
            transitive_infos = depset(
                direct = dep_bindings_infos,
                transitive = [
                    getattr(dep, "transitive_infos", depset())
                    for dep in dep_bindings_infos
                ],
            ),
        ),
        bindings_info,
        OutputGroupInfo(out = output_depset, out_compiled = out_compiled),
    ]

# These are shared between cc_bindings_from_rust_aspect and cpp_api_from_rust_toolchain_bindings.
private_common_attrs = {
    "_clang_format": attr.label(
        default = Label("@llvm_toolchain//:clang-format"),
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_cc_deps_for_bindings": attr.label_list(
        doc = "Dependencies needed to build the C++ sources generated by cc_bindings_from_rs.",
        default = [
            "//support/internal:bindings_support",
            "//support:annotations_internal",
            "//support:lifetime_annotations",
            "//support:movable",
            "//support/rs:char",
            "//support/rs:enum",
            "//support/rs:int",
            "//support/rs:slice_ref",
            "//support/rs:str_ref",
            "//support/rs:fn_ref",
            "//support/rs:fn",
            "//support:bridge_cpp",
            "//support/rs:traits",
            "//support/rs:option",
            "//support/rs:result",
            "//support/rs:tuple",
            "//support/rs:unit",
            "//support/rs:vec",
            "//support/rs:box",
            "//support/rs:pin_box",
            "//support/rs:slice_box",
            "//support/rs:str_box",
            "//support/protobuf:rust",
            "//support/rs:iterator_adapter",
            "//support/rs:dyn_erased_future_cc",
            "//common:include_allocator_symbols"
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
            "//support:crubit_support",
            "//support:hash_rust",
        ],
    ),
    "_rustfmt": attr.label(
        default = "@rules_rust//tools/upstream_wrapper:rustfmt",
        executable = True,
        allow_single_file = True,
        cfg = "exec",
    ),
    "_rustfmt_cfg": attr.label(
        default = "//:rustfmt.toml",
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
    "_is_golden_test": attr.label(
        default = "//common/bazel_support:is_golden_test",
    ),
    "_globally_enabled_features": attr.label(
        default = "//common/bazel_support:globally_enabled_features",
    ),
    "_crubit_feature_denylists": attr.label(
        default = "//features/denylists:supported_feature_denylists",
    ),
    "_verbose_log_targets": attr.label(
        default = "//common/bazel_support:verbose_log_targets",
    ),
}

cc_bindings_from_rust_aspect = aspect(
    implementation = _cc_bindings_from_rust_aspect_impl,
    doc = "Aspect for generating C++ bindings for a Rust library.",
    attr_aspects = ["deps"],
    attrs = private_common_attrs | {
        "_stdlib_bindings": attr.label(
            doc = "Standard library crates (core, alloc, std). These are implicitly imported by all bindings.",
            default = "//support/rs_std:stdlib",
        ),
    },
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        config_common.toolchain_type("//cc_bindings_from_rs/bazel_support:toolchain_type", mandatory = False),
    ] + use_cpp_toolchain(),
    fragments = ["cpp"],
)

def _make_round_trip_rust_bindings_info(label, headers, dep_variant_info, dep_targets = []):
    """Creates a `RustBindingsFromCcInfo` so `rs_bindings_from_cc` recognizes Rust-defined headers."""
    return RustBindingsFromCcInfo(
        cc_info = None,
        dep_variant_info = dep_variant_info,
        pass_through_dep_variant_infos = depset(),
        target_args = depset(
            direct = [
                # Mark the headers as being owned by the (bindings for the) crate, so that
                # rs_bindings_from_cc doesn't assume that they're owned by the current target or
                # that they don't have bindings.
                json.encode(
                    {
                        "t": str(label),
                        "h": [h.path for h in headers],
                        # Note: the feature set is a lie, crubit won't run on this target.
                        # The "pun" we're relying on is that Crubit will think that
                        # "everything will get bindings if I run rs_bindings_from_cc on it".
                        # In fact, we _won't_ run rs_bindings_from_cc on it, but everything "will
                        # get bindings" because it was originally defined in Rust to begin with.
                        # TODO(jeanpierreda): Maybe define an assume_bindings/round_trip feature?
                        "f": ["all"],
                    },
                ),
            ],
            transitive = [
                dep[RustBindingsFromCcInfo].target_args
                for dep in dep_targets
                if RustBindingsFromCcInfo in dep
            ],
        ),
        namespaces = None,
        additional_rust_srcs = depset(),
    )

def _cc_bindings_from_rust_rule_impl(ctx):
    crate = ctx.attr.crate
    providers = [
        crate[CcBindingsFromRustInfo].cc_info,
        # If we try to generate rust bindings of c++ bindings of this rust crate, we get back
        # the original rust crate again.
        _make_round_trip_rust_bindings_info(
            label = crate.label,
            headers = crate[CcBindingsFromRustInfo].headers,
            dep_variant_info = DepVariantInfo(
                crate_info = crate[CrateInfo] if CrateInfo in crate else None,
                dep_info = crate[DepInfo] if DepInfo in crate else None,
                build_info = crate[BuildInfo] if BuildInfo in crate else None,
                cc_info = crate[CcInfo] if CcInfo in crate else None,
            ),
        ),
    ]
    if OutputGroupInfo in crate:
        providers.append(DefaultInfo(files = crate[OutputGroupInfo].out))
    return providers

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

cpp_api_from_rust = cc_bindings_from_rust

cc_bindings_from_rs = cc_bindings_from_rust

def _cpp_api_from_rust_toolchain_bindings_impl(ctx):
    crate_name = ctx.attr.crate_name
    basename = ctx.label.name

    toolchain = find_toolchain(ctx)
    cc_toolchain = find_cpp_toolchain(ctx)
    feature_configuration = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc_toolchain,
    )

    dep_info, build_info, linkstamps = collect_deps(
        deps = [],
        proc_macro_deps = [],
        aliases = {},
    )

    # This will be a depset of all the .rlib/.rmeta files in the stdlib.
    srcs = toolchain.rust_std
    deps = ctx.attr.deps

    crate_info = rust_common.create_crate_info(
        name = crate_name,
        compile_data = depset(),
        edition = toolchain.default_edition,
        owner = ctx.label,
        srcs = depset(),
        type = "rlib",
    )

    compile_inputs, _, _, _, _, _ = collect_inputs(
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

    args = _build_crubit_rustc_args(ctx, toolchain)

    env = {
        "CARGO_PKG_NAME": crate_name,
    }

    dep_bindings_infos = _get_dep_bindings_infos(ctx.attr)

    config = crate_name_to_library_config(aspect_hints = [], rust_infos = dep_bindings_infos)
    if ctx.attr.cpp_namespace:
        config["self"] = struct(namespace = ctx.attr.cpp_namespace)
        config[crate_name] = struct(namespace = ctx.attr.cpp_namespace)
    bindings_info, features, config, output_depset = _generate_bindings(
        ctx,
        dep_bindings_infos = dep_bindings_infos,
        # Hardcode renaming of generated C++ namespaces so they don't conflict with existing C++
        # namespaces.
        config = config,
        label = ctx.label,
        features = [],
        cli_flags = [],
        crate_name = crate_name,
        basename = basename,
        inputs = depset(transitive = [srcs, compile_inputs]),
        args = args,
        rustc_env = env,
        proto_crate_renames = {},
        self_rmeta = None,
        is_golden_test_override = False,
    )

    dep_variant_info = _compile_rs_out_file(ctx, ctx.attr, bindings_info.rust_file, crate_name, [])

    (extra_cc_hdrs, extra_cc_srcs) = get_additional_cc_hdrs_and_srcs([])

    cc_info, extra_cc_library_to_link = _make_cc_info_for_h_out_file(
        ctx,
        bindings_info.h_file,
        extra_cc_hdrs,
        extra_cc_srcs,
        cc_infos = [dep_variant_info.cc_info] + [
            dep_bindings_info.cc_info
            for dep_bindings_info in dep_bindings_infos
        ],
    )

    out_compiled = depset(
        [dep_variant_info.crate_info.output] +
        get_static_library_files(extra_cc_library_to_link),
    )

    self_info = CcBindingsFromRustInfo(
        cc_info = cc_info,
        crate_key = crate_name,
        headers = [bindings_info.h_file],
        features = features,
        configuration = config,
        transitive_infos = depset(
            direct = dep_bindings_infos,
            transitive = [
                getattr(dep, "transitive_infos", depset())
                for dep in dep_bindings_infos
            ],
        ),
    )

    rust_bindings_from_cc_info = _make_round_trip_rust_bindings_info(
        label = ctx.label,
        headers = [bindings_info.h_file],
        dep_variant_info = None,
        dep_targets = ctx.attr.deps,
    )

    return [
        self_info,
        bindings_info,
        cc_info,
        rust_bindings_from_cc_info,
        OutputGroupInfo(out = output_depset, out_compiled = out_compiled),
    ]

cpp_api_from_rust_toolchain_bindings = rule(
    implementation = _cpp_api_from_rust_toolchain_bindings_impl,
    doc = "Rule for generating C++ bindings for Rust code that lives in the toolchain. Notably this includes std, core, and alloc.",
    attrs = private_common_attrs | {
        "crate_name": attr.string(
            doc = "The name of the Rust crate.",
            mandatory = True,
        ),
        "deps": attr.label_list(
            doc = "Dependencies needed to build the Rust sources generated by cc_bindings_from_rs.",
            mandatory = False,
        ),
        "cpp_namespace": attr.string(
            doc = "The namespace to use for the generated C++ bindings.",
            mandatory = False,
        ),
    },
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        config_common.toolchain_type("//cc_bindings_from_rs/bazel_support:toolchain_type", mandatory = False),
    ] + use_cpp_toolchain(),
    fragments = ["cpp"],
)
