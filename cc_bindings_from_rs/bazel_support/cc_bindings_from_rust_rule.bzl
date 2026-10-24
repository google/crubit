# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cc_bindings_from_rust` rule."""

load("@rules_cc//cc/common:cc_common.bzl", "cc_common")
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
    "AliasableDepInfo",
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
    "get_static_libraries_from_cc_info",
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
    "//rs_bindings_from_cc/bazel_support:encode_raw_string_as_crate_name.bzl",
    "encode_raw_string_as_crate_name",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)

load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain", "use_cpp_toolchain")

# Targets which do not receive C++ bindings at all.
targets_to_remove = [
]

def _encode_crate_key(crate_info, legacy_mangling_allowlist):
    """Returns the label-encoded crate name for `crate_info`, or its plain name if exempt."""
    owner_str = str(crate_info.owner)
    if owner_str in legacy_mangling_allowlist or owner_str.endswith("/support/cc_std:cc_std"):
        return crate_info.name
    return encode_raw_string_as_crate_name(owner_str)

def _dep_crate_key(dep_bindings_info, use_label_encoded_names_for_deps):
    return dep_bindings_info.encoded_crate_key if use_label_encoded_names_for_deps else dep_bindings_info.crate_key

def _aliasable_dep_to_file(aliasable_dep):
    crate_info = aliasable_dep.dep
    if crate_info.metadata and getattr(crate_info, "metadata_supports_pipelining", True):
        return crate_info.metadata
    return crate_info.output

def _aliasable_dep_to_extern_flag(aliasable_dep):
    return "--extern={}={}".format(aliasable_dep.name, _aliasable_dep_to_file(aliasable_dep).path)

def _crates_to_aliasable_deps(crates, legacy_mangling_allowlist):
    """Wraps each non-proc-macro `CrateInfo` in an `AliasableDepInfo` under its encoded name.

    `crates` may contain either bare `CrateInfo`s or `AliasableDepInfo`s (as found in
    `DepInfo.transitive_crates`); the latter are unwrapped first.
    """
    result = []
    for raw in crates:
        c = raw.dep if hasattr(raw, "dep") else raw
        if "proc-macro" not in (c.type, getattr(c, "wrapped_crate_type", None)):
            result.append(AliasableDepInfo(
                name = _encode_crate_key(c, legacy_mangling_allowlist),
                dep = c,
            ))
    return result

def _collect_dep_extra_named_deps(target, attr, dep_info, legacy_mangling_allowlist):
    """Returns a depset of `AliasableDepInfo`s for every Rust crate reachable from `target`'s deps."""
    direct = []
    transitive = []

    for dep in getattr(attr, "deps", []) + getattr(attr, "cc_deps", []):
        if CcBindingsFromRustInfo in dep:
            transitive.append(dep[CcBindingsFromRustInfo].extra_named_deps)
        elif RustBindingsFromCcInfo in dep and dep[RustBindingsFromCcInfo].dep_variant_info != None:
            transitive.append(getattr(dep[RustBindingsFromCcInfo], "extra_named_deps", depset()))
        elif CrateInfo in dep:
            direct.extend(_crates_to_aliasable_deps(
                [dep[CrateInfo]] + dep[DepInfo].transitive_crates.to_list(),
                legacy_mangling_allowlist,
            ))
    return depset(direct = direct, transitive = transitive)

def _make_target_args(label, h_file, dep_bindings_infos):
    # Mark the headers as being owned by the (bindings for the) crate, so that
    # rs_bindings_from_cc doesn't assume that they're owned by the current target or
    # that they don't have bindings.
    return depset(
        direct = [
            json.encode({
                "t": str(label),
                "h": [h_file.path],
                # Note: the feature set is a lie, crubit won't run on this target.
                # The "pun" we're relying on is that Crubit will think that
                # "everything will get bindings if I run rs_bindings_from_cc on it".
                # In fact, we _won't_ run rs_bindings_from_cc on it, but everything "will
                # get bindings" because it was originally defined in Rust to begin with.
                # TODO(jeanpierreda): Maybe define an assume_bindings/round_trip feature?
                "f": ["all"],
            }),
        ],
        transitive = [dep.target_args for dep in dep_bindings_infos],
    )

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

def _rustc_lib_env(ctx):
    """Returns an environment that sets the dylib search path to include rustc libraries.

    This is needed in bazel where rustc_private dynamically links against librustc_driver.
    Internally, we statically link against the compiler so this isn't an issue.  If the path to
    rustc's libraries cannot be determined this returns an empty dictionary.

    Args:
      ctx: The rule context.

    Returns:
      A dictionary of environment variables to set.
    """
    rust_toolchain = ctx.toolchains["@rules_rust//rust:toolchain_type"]
    if rust_toolchain == None:
        return {}
    rustc_lib = rust_toolchain.rustc_lib.to_list()
    if len(rustc_lib) <= 0:
        return {}
    return {
        "LD_LIBRARY_PATH": rustc_lib[0].dirname,
    }

def _get_crate_dirname(crate):
    """A helper macro for getting the directory name of the current crate's output path.

    This must be a function because `map_each` does not accept a lambda.

    Args:
        crate (CrateInfo|AliasableDepInfo): A CrateInfo or an AliasableDepInfo provider

    Returns:
        str: The directory name of the the output File that will be produced.
    """
    if hasattr(crate, "dep"):
        return _aliasable_dep_to_file(crate).dirname
    return crate.output.dirname

def _filter_crubit_rustc_args(ctx, toolchain, dep_info, dep_extra_named_deps = depset()):
    """Returns an `Args` object containing the crubit-supported rustc flags for the bindings crate.

    Args:
      ctx: The rule context.
      toolchain: The rust toolchain.
      dep_info: The dep info.
      dep_extra_named_deps: Additional `AliasableDepInfo`s to include in `-Ldependency`.
    Returns:
      An `Args` object containing our filtered `rustc` flags.
    """
    rustc_flags = ctx.actions.args()
    rustc_flags.add_all(toolchain.rust_std_paths, before_each = "-L", format_each = "%s")
    rustc_flags.add(toolchain.target_flag_value, format = "--target=%s")
    crates = depset(
        transitive = [
            dep_info.direct_crates,
            dep_info.transitive_crates,
            dep_extra_named_deps,
        ],
    )
    rustc_flags.add_all(
        crates,
        map_each = _get_crate_dirname,
        uniquify = True,
        format_each = "-Ldependency=%s",
    )
    if toolchain._toolchain_generated_sysroot:
        rustc_flags.add(toolchain.sysroot, format = "--sysroot=%s")
    return rustc_flags

def _generate_bindings(ctx, dep_bindings_infos, config, label, features, cli_flags, crate_name, basename, inputs, process_wrapper_flags, rustc_flags, rustc_env, proto_crate_renames, self_rmeta, dep_extra_named_deps = depset(), use_label_encoded_names_for_deps = False, is_golden_test_override = None, target_denied_features = []):
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
      process_wrapper_flags: `Args` object of `process_wrapper` arguments.
      rustc_flags: `Args` object of `rustc` flags.
      rustc_env: `rustc` environment to use when running `cc_bindings_from_rs`
      proto_crate_renames: Mapping of the `rust_proto_library` to the `proto_library` crate name.
      self_rmeta: The rmeta file for the current crate.
      dep_extra_named_deps: Depset of `AliasableDepInfo`s for label-encoded dependencies.
      use_label_encoded_names_for_deps: Whether dependency crate keys use label-encoded names.
      is_golden_test_override: Overrides value of `--is-golden-test` flag instead of checking for the transition.
      target_denied_features: Features denied for this target by `SUPPORTED_FEATURE_DENYLISTS`, overriding `--default-features`.

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
        dep_key = _dep_crate_key(dep_bindings_info, use_label_encoded_names_for_deps)
        for header in dep_bindings_info.headers:
            crubit_args.add("--crate-header", dep_key + "=" + header.short_path)
        for feature in dep_bindings_info.features:
            crubit_args.add("--crate-feature", dep_key + "=" + feature)

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
        dep_key = _dep_crate_key(dep_bindings_info, use_label_encoded_names_for_deps)
        for feature in getattr(dep_bindings_info, "target_denied_features", []):
            if feature not in unsupported_features:
                crubit_args.add("--crate-disabled-feature", dep_key + "=" + feature)
    for feature in target_denied_features:
        if feature not in unsupported_features:
            crubit_args.add("--crate-disabled-feature", "self=" + feature)

    outputs = [h_out_file, rs_out_file]
    if ctx.attr._generate_error_report[BuildSettingInfo].value:
        error_report_output = ctx.actions.declare_file(basename + "_cc_api_error_report.json")
        crubit_args.add(
            "--error-report-out",
            error_report_output.path,
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
    if self_crate_name == None:
        self_crate_name = crate_name
        crubit_args.add("--source-crate-name", self_crate_name)
        if self_rmeta != None:
            crubit_args.add("--extern={}={}".format(self_crate_name, self_rmeta.path))

    crubit_args.add_all(dep_extra_named_deps, uniquify = True, map_each = _aliasable_dep_to_extern_flag)

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
        tool_args.add("--")
        tool_args.add(toolchain.binary)
        ctx.actions.run(
            outputs = outputs,
            inputs = depset(
                [f for f in [ctx.file._rustfmt_cfg, self_rmeta] if f != None] + [
                    _aliasable_dep_to_file(d)
                    for d in dep_extra_named_deps.to_list()
                ],
                transitive = [inputs],
            ),
            env = rustc_env | verbose_log_env | _rustc_lib_env(ctx),
            tools = [toolchain.binary, ctx.executable._rustfmt, ctx.executable._clang_format],
            executable = ctx.executable._process_wrapper,
            mnemonic = "CcBindingsFromRust",
            progress_message = "Generating C++ bindings from Rust: %s" % h_out_file,
            # We don't pass `construct_arguments(...).all` here, because we want to replace the
            # rustc path with the path to crubit.
            #
            # That said, if we passed arguments to crubit via environment variables or via flags that
            # can be interleaved with rustc flags in any order, and if we used toolchain.binary
            # as the tool_path for construct_arguments, then this could be `all` instead.
            arguments = [process_wrapper_flags, tool_args, crubit_args, rustc_flags],
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
      extra_cc_hdrs: Additional C++ header files to expose as public headers.
      extra_cc_srcs: Additional C++ source files to compile with the header.
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

    return CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
    )

def _compile_rs_out_file(ctx, attr, rs_out_file, crate_name, deps, aliases = {}, extra_named_deps = None):
    """Compiles the generated "..._cc_api_impl.rs" file.

    Args:
      ctx: The rule context.
      attr: The rule attributes.
      rs_out_file: The generated "..._cc_api_impl.rs" file
      crate_name: The name of the crate.
      deps: The dependencies of the target crate.
      aliases: Crate aliases passed to `compile_rust`.
      extra_named_deps: Optional depset of `AliasableDepInfo`s for label-encoded dependencies.

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

    if extra_named_deps == None:
        # The `..._cc_api_impl.rs` file needs to depend on all the deps of the target crate.
        _deps += [dep for target in deps for dep in target[CrateInfo].deps.to_list()]
        extra_named_deps = depset()

    return compile_rust(
        ctx,
        attr = attr,
        src = rs_out_file,
        extra_srcs = [],
        deps = depset(_deps),
        crate_name = crate_name + "_cc_api_impl",
        include_coverage = True,
        aliases = aliases,
        extra_named_deps = extra_named_deps,
    )

def _cc_bindings_from_rust_aspect_impl(target, ctx):
    basename = target.label.name

    if CrateInfo not in target:
        return []
    if CcBindingsFromRustInfo in target:
        return []
    if str(target.label) in targets_to_remove:
        return []

    cc_bindings_toolchain = ctx.toolchains["//cc_bindings_from_rs/bazel_support:toolchain_type"]
    toolchain_supports_label_encoded_deps = (
        cc_bindings_toolchain != None and
        cc_bindings_toolchain.cc_bindings_from_rs_toolchain_info.supports_label_encoded_deps
    )
    legacy_mangling_allowlist = ctx.attr._legacy_mangling_allowlist[BuildSettingInfo].value

    # `enable_label_encoded_deps` controls whether label-encoded `extra_named_deps` are collected
    # and propagated to dependents at all; `use_label_encoded_names_for_deps` controls whether
    # *this* target's bindings are generated with label-encoded names. Allowlisted targets keep
    # plain names themselves but still propagate `extra_named_deps` for their dependents.
    enable_label_encoded_deps = (
        ctx.attr._use_label_encoded_names_for_deps[BuildSettingInfo].value and
        toolchain_supports_label_encoded_deps and
        not ctx.attr._is_golden_test[BuildSettingInfo].value
    )
    use_label_encoded_names_for_deps = (
        enable_label_encoded_deps and
        str(target.label) not in legacy_mangling_allowlist
    )

    crate_info = target[CrateInfo]
    encoded_crate_key = _encode_crate_key(crate_info, legacy_mangling_allowlist)
    self_crate_key = encoded_crate_key if use_label_encoded_names_for_deps else crate_info.name

    proto_crate_renames = []

    toolchain = find_toolchain(ctx)
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

    compile_inputs, out_dir, build_env_files, _, linkstamp_outs, ambiguous_libs = collect_inputs(
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

    dep_extra_named_deps = (
        _collect_dep_extra_named_deps(target, ctx.rule.attr, dep_info, legacy_mangling_allowlist) if enable_label_encoded_deps else depset()
    )
    active_extra_named_deps = dep_extra_named_deps if use_label_encoded_names_for_deps else depset()

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
    rustc_flags = _filter_crubit_rustc_args(ctx, toolchain, dep_info, active_extra_named_deps)

    if target[CrateInfo].metadata != None:
        rmeta = target[CrateInfo].metadata
    else:
        # Expectation is this will be an rlib file.
        rmeta = target[CrateInfo].output

    features = find_crubit_features(target, ctx)
    target_denied_features = find_denied_crubit_features(target, ctx)
    cli_flags = collect_cc_bindings_from_rust_cli_flags(target, ctx)

    ignore_symbols_from_files = depset(transitive = [
        dep[RustBindingsFromCcInfo].additional_rust_srcs
        for dep in getattr(ctx.rule.attr, "deps", []) + getattr(ctx.rule.attr, "cc_deps", [])
        if RustBindingsFromCcInfo in dep
    ])
    for file in ignore_symbols_from_files.to_list():
        # These are internal to Crubit and we want them receive bindings despite being in additional
        # sources.
        if "cc_std_impl" in file.path or "cc_std_impl" in file.short_path:
            continue
        cli_flags.append("--ignore-symbols-from-files=" + file.path)
    stdlib_info = ctx.attr._stdlib_bindings[CcBindingsFromRustInfo]
    stdlib_bindings_infos = [stdlib_info] + stdlib_info.transitive_infos.to_list()
    user_dep_bindings_infos = _get_dep_bindings_infos(ctx.rule.attr)
    dep_bindings_infos = user_dep_bindings_infos + stdlib_bindings_infos

    config = crate_name_to_library_config(
        aspect_hints = ctx.rule.attr.aspect_hints,
        rust_infos = dep_bindings_infos,
        use_label_encoded_names_for_deps = use_label_encoded_names_for_deps,
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
        crate_name = self_crate_key,
        basename = basename,
        inputs = compile_inputs,
        process_wrapper_flags = original_args.process_wrapper_flags,
        rustc_flags = rustc_flags,
        rustc_env = env,
        proto_crate_renames = proto_crate_renames,
        self_rmeta = rmeta,
        dep_extra_named_deps = active_extra_named_deps,
        use_label_encoded_names_for_deps = use_label_encoded_names_for_deps,
        target_denied_features = target_denied_features,
    )

    dep_variant_info = _compile_rs_out_file(
        ctx,
        ctx.rule.attr,
        bindings_info.rust_file,
        crate_info.name,
        [target],
        aliases = {target: self_crate_key} if use_label_encoded_names_for_deps else {},
        extra_named_deps = dep_extra_named_deps if use_label_encoded_names_for_deps else None,
    )

    cc_info = _make_cc_info_for_h_out_file(
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
        get_static_libraries_from_cc_info(cc_info),
    )

    return [
        CcBindingsFromRustInfo(
            cc_info = cc_info,
            crate_key = crate_info.name,
            encoded_crate_key = encoded_crate_key,
            headers = [bindings_info.h_file],
            features = features,
            target_args = _make_target_args(target.label, bindings_info.h_file, user_dep_bindings_infos),
            target_denied_features = target_denied_features,
            configuration = config,
            transitive_infos = depset(
                direct = dep_bindings_infos,
                transitive = [dep.transitive_infos for dep in dep_bindings_infos],
            ),
            extra_named_deps = depset(
                direct = [AliasableDepInfo(name = encoded_crate_key, dep = crate_info)],
                transitive = [dep_extra_named_deps],
            ) if enable_label_encoded_deps else depset(),
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
            "//support/rs_std:char",
            "//support/rs_std:enum",
            "//support/rs_std:int",
            "//support/rs_std:slice_ref",
            "//support/rs_std:str_ref",
            "//support/rs_std:fn_ref",
            "//support/rs_std:fn",
            "//support:bridge_cpp",
            "//support/rs_std:traits",
            "//support/rs_std:option",
            "//support/rs_std:result",
            "//support/rs_std:tuple",
            "//support/rs_std:unit",
            "//support/rs_std:vec",
            "//support/protobuf:rust",
            "//support/rs_std:iterator_adapter",
            "//support/rs_std:dyn_erased_future_cc",
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
    "_use_label_encoded_names_for_deps": attr.label(
        default = "//common/bazel_support:use_label_encoded_names_for_deps",
    ),
    "_legacy_mangling_allowlist": attr.label(
        default = "//rs_bindings_from_cc/bazel_support:legacy_mangling_allowlist",
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

def _cc_bindings_from_rust_rule_impl(ctx):
    crate = ctx.attr.crate
    providers = [
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
            extra_named_deps = crate[CcBindingsFromRustInfo].extra_named_deps,
            pass_through_dep_variant_infos = depset(),
            target_args = crate[CcBindingsFromRustInfo].target_args,
            namespaces = None,
            additional_rust_srcs = depset(),
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

    rustc_flags = ctx.actions.args()
    rustc_flags.add_all(toolchain.rust_std_paths, before_each = "-L", format_each = "%s")
    rustc_flags.add(toolchain.target_flag_value, format = "--target=%s")

    # Ensure the sysroot is set for the target platform
    if toolchain._toolchain_generated_sysroot:
        rustc_flags.add(toolchain.sysroot, format = "--sysroot=%s")

    process_wrapper_flags = ctx.actions.args()

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
        process_wrapper_flags = process_wrapper_flags,
        rustc_flags = rustc_flags,
        rustc_env = env,
        proto_crate_renames = {},
        self_rmeta = None,
        is_golden_test_override = False,
    )

    dep_variant_info = _compile_rs_out_file(ctx, ctx.attr, bindings_info.rust_file, crate_name, [])

    (extra_cc_hdrs, extra_cc_srcs) = get_additional_cc_hdrs_and_srcs([])

    cc_info = _make_cc_info_for_h_out_file(
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
        get_static_libraries_from_cc_info(cc_info),
    )

    self_info = CcBindingsFromRustInfo(
        cc_info = cc_info,
        crate_key = crate_name,
        encoded_crate_key = crate_name,
        headers = [bindings_info.h_file],
        features = features,
        target_args = _make_target_args(ctx.label, bindings_info.h_file, dep_bindings_infos),
        configuration = config,
        transitive_infos = depset(
            direct = dep_bindings_infos,
            transitive = [
                getattr(dep, "transitive_infos", depset())
                for dep in dep_bindings_infos
            ],
        ),
        extra_named_deps = depset(),
    )

    return [
        self_info,
        bindings_info,
        cc_info,
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
