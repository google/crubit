# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.
"""

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:common.bzl",
    "rust_common",
)

def compile_rust(ctx, attr, src, extra_srcs, deps, crate_name, include_coverage, force_all_deps_direct, allow_lto = True, aliases = {}):
    """Compiles a Rust source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      src: The source file to be compiled.
      extra_srcs: Additional source files to include in the crate.
      deps: depset[DepVariantInfo]: A depset of dependencies needed.
      crate_name: (string) crate name for naming the output files (.rlib, .rmeta...))
      include_coverage: (bool) Whether or not coverage information should be generated.
      force_all_deps_direct: (bool) Whether or not to force all deps to be direct.
      allow_lto: (bool, optional) Whether to allow LTO
      aliases: (dict, optional) A dict of aliases to be passed to the rustc_compile_action.

    Returns:
      A DepVariantInfo provider.
    """

    emit_rmeta = True

    return rust_common.compile_rust(
        ctx = ctx,
        attr = attr,
        src = src,
        extra_srcs = extra_srcs,
        deps = deps,
        edition = "2018",
        emit_rmeta = emit_rmeta,
        crate_name = crate_name,
        aliases = aliases,
        include_coverage = include_coverage,
        allow_lto = allow_lto,
        force_all_deps_direct = force_all_deps_direct,
        # LINT.IfChange
        rust_flags = ["-Zallow-features=custom_inner_attributes,impl_trait_in_assoc_type,register_tool,negative_impls,extern_types,arbitrary_self_types,allocator_api,cfg_sanitize"],
        # LINT.ThenChange(//docs/overview/unstable_features.md)
    )
