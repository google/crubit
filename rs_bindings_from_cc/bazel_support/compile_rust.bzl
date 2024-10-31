# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.
"""

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:providers.bzl", "DepVariantInfo")

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:rustc.bzl",
    "rustc_compile_action",
)
load("@bazel_skylib//lib:structs.bzl", "structs")

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

def compile_rust(ctx, attr, src, extra_srcs, deps, crate_name, include_coverage, force_all_deps_direct):
    """Compiles a Rust source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      src: The source file to be compiled.
      extra_srcs: Additional source files to include in the crate.
      deps: List[DepVariantInfo]: A list of dependencies needed.
      crate_name: (string) crate name for naming the output files (.rlib, .rmeta...))
      include_coverage: (bool) Whether or not coverage information should be generated.
      force_all_deps_direct: (bool) Whether or not to force all deps to be direct.

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
    attr_args["rustc_flags"] = []
    providers = rustc_compile_action(
        ctx = ctx,
        attr = struct(**attr_args),
        toolchain = toolchain,
        crate_info_dict = dict(
            name = crate_name,
            type = "rlib",
            root = src,
            srcs = depset([src] + extra_srcs),
            deps = depset(deps),
            proc_macro_deps = depset([]),
            aliases = {},
            output = lib,
            metadata = rmeta,
            edition = "2018",
            is_test = False,
            rustc_env = {},
            compile_data = depset([]),
            compile_data_targets = depset([]),
            owner = ctx.label,
        ),
        rust_flags = ["-Zallow-features=custom_inner_attributes,impl_trait_in_assoc_type,register_tool,negative_impls,vec_into_raw_parts,extern_types,arbitrary_self_types,allocator_api"],
        output_hash = output_hash,
        force_all_deps_direct = force_all_deps_direct,
        include_coverage = include_coverage,
    )

    return DepVariantInfo(
        crate_info = _get_crate_info(providers),
        dep_info = _get_dep_info(providers),
        cc_info = _get_cc_info(providers),
        build_info = None,
    )
