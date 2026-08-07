# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.
"""

load("@rules_rust//:version.bzl", RUST_VERSION = "VERSION")

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:providers.bzl", "DepVariantInfo")

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:rustc.bzl",
    "rustc_compile_action",
)

load("@bazel_skylib//lib:structs.bzl", "structs")

_extra_rustc_flags = [
    # b/540237308: Hide thunk symbols to avoid unnecessary GOT/PLT overhead.
    "-Zdefault-visibility=hidden",
]

def _version_parts(version):
    major, minor = version.split(".")[0:2]
    return (int(major), int(minor))

def _rust_version_ge(version):
    """Checks if the rust version as at least the given major.minor version."""
    return _version_parts(RUST_VERSION) >= _version_parts(version)

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

def _get_cc_info(providers):
    for provider in providers:
        if hasattr(provider, "linking_context"):
            return provider
    fail("Couldn't find a CcInfo in the list of providers")

def _filter_rustc_flags(flags):
    ALLOWED_PREFIXES = [
        # Flags that logically apply to the whole program.
        "-Ccode-model=",
    ]
    return [
        flag
        for flag in flags
        if any([
            flag.startswith(prefix)
            for prefix in ALLOWED_PREFIXES
        ])
    ]

def compile_rust(ctx, attr, src, extra_srcs, deps, crate_name, include_coverage, allow_lto = True, aliases = {}, remap_path_prefix = {}, extra_named_deps = depset()):
    """Compiles a Rust source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      src: The source file to be compiled.
      extra_srcs: Additional source files to include in the crate.
      deps: depset[DepVariantInfo]: A depset of dependencies needed.
      crate_name: (string) crate name for naming the output files (.rlib, .rmeta...))
      include_coverage: (bool) Whether or not coverage information should be generated.
      allow_lto: (bool, optional) Whether to allow LTO
      aliases: (dict, optional) A dict of aliases to be passed to the rustc_compile_action.
      remap_path_prefix: (dict, optional) A dict of {symlink_path: source_path} to be remapped by rustc.
      extra_named_deps: (depset[AliasableDepInfo], optional) Extra dependencies with custom crate names used for the compilation of the generated bindings.

    Returns:
      A DepVariantInfo provider.
    """
    toolchain = ctx.toolchains["@rules_rust//rust:toolchain_type"]

    output_hash = repr(hash(src.path))

    lib_name = "{prefix}{name}-{lib_hash}{extension}".format(
        prefix = "lib",
        name = crate_name,
        lib_hash = output_hash,
        extension = ".rlib",
    )

    rmeta_name = "{prefix}{name}-{lib_hash}{extension}".format(
        prefix = "lib",
        name = crate_name,
        lib_hash = output_hash,
        extension = ".rmeta",
    )

    lib = ctx.actions.declare_file(lib_name)

    rmeta = ctx.actions.declare_file(rmeta_name)

    # TODO(b/336367148): We should inherit almost nothing from `attr`, but for now, at least, we
    # should omit the rustc_flags.
    attr_args = structs.to_dict(attr)
    if "rustc_flags" in attr_args:
        attr_args["rustc_flags"] = _filter_rustc_flags(attr_args["rustc_flags"])

    if _rust_version_ge("0.67"):
        srcs = [src] + extra_srcs
    else:
        srcs = depset([src] + extra_srcs)

    remapped_flags = []
    for symlink, source in remap_path_prefix.items():
        remapped_flags.append("--remap-path-prefix={}={}".format(symlink, source))

    providers = rustc_compile_action(
        ctx = ctx,
        attr = struct(**attr_args),
        toolchain = toolchain,
        crate_info_dict = dict(
            name = crate_name,
            type = "rlib",
            root = src,
            srcs = srcs,
            deps = deps.to_list(),
            proc_macro_deps = [],
            aliases = aliases,
            output = lib,
            metadata = rmeta,
            edition = "2024",
            is_test = False,
            rustc_env = {},
            compile_data = depset([]),
            compile_data_targets = depset([]),
            owner = ctx.label,
        ),
        rust_flags = remapped_flags + _extra_rustc_flags,
        output_hash = output_hash,
        include_coverage = include_coverage,
        # LINT.IfChange
        allowed_unstable_rust_features = [
            # <internal link> start
            "allocator_api",
            "arbitrary_self_types",
            "cfg_sanitize",
            "custom_inner_attributes",
            "extern_types",
            "impl_trait_in_assoc_type",
            "negative_impls",
            "register_tool",
            # <internal link> end
        ],
        # LINT.ThenChange(//docs/overview/unstable_features.md)
    )

    return DepVariantInfo(
        crate_info = _get_crate_info(providers),
        dep_info = _get_dep_info(providers),
        cc_info = _get_cc_info(providers),
        build_info = None,
    )
