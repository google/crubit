# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility module for sharing logic between rules and aspects that generate Rust bindings from C++.
"""

load("@rules_cc//cc/common:cc_common.bzl", "cc_common")
load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:generate_bindings.bzl", "escape_cpp_target_name")

def compile_cc(
        ctx,
        attr,
        cc_toolchain,
        feature_configuration,
        src,
        cc_infos,
        extra_cc_compilation_action_inputs,
        extra_hdrs = []):
    """Compiles a C++ source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      cc_toolchain: A cc_toolchain.
      feature_configuration: A feature configuration.
      src: The source file to be compiled.
      cc_infos: List[CcInfo]: A list of CcInfo dependencies. They are treated as exported deps of
        `src`: `src` may include their headers and the headers of their exported deps under
        `layering_check`, and consume their precompiled header modules.
      extra_cc_compilation_action_inputs: A list of input files for the C++ compilation action.
      extra_hdrs: A list of headers owned by this compilation action (passed as `public_hdrs`).

    Returns:
      A CcInfo provider.
    """

    # Use `direct_cc_infos` (exported deps) rather than `cc_infos` (non-exported deps) so that
    # `_module_map` and `_exporting_module_maps` are preserved for `layering_check` and
    # `use_header_modules`.
    cc_info = cc_common.merge_cc_infos(direct_cc_infos = cc_infos)

    user_copts = []
    for copt in getattr(attr, "copts", []):
        # ctx.expand_make_variables is deprecated, but its replacement ctx.var does not suffice.
        user_copts.append(ctx.expand_make_variables("copts", copt, {}))

    local_defines = []
    for d in getattr(attr, "local_defines", []):
        local_defines.append(ctx.expand_make_variables("local_defines", d, {}))

    (compilation_context, compilation_outputs) = cc_common.compile(
        name = src.basename,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        srcs = [src],
        public_hdrs = extra_hdrs,
        additional_inputs = extra_cc_compilation_action_inputs,
        user_compile_flags = user_copts,
        local_defines = local_defines,
        compilation_contexts = [cc_info.compilation_context],
    )

    # We need to allow "backwards" dependencies from the impl library onto Rust-generated thunk
    # code, because this is used to implement Rust callbacks from C++.
    # TODO(b/468327990): Make this portable in OSS.
    backrefs_flag = []
    if "linux" in cc_toolchain.target_gnu_system_name:
        target = escape_cpp_target_name(ctx.label.package, ctx.label.name)
        backrefs_flag.append("-Wl,--warn-backrefs-exclude=*/{package}/lib{target}-*".format(package = ctx.label.package, target = target))

        # With ThinLTO, the crate is linked as a bare object (`{target}-{hash}.o`, or
        # `{target}-{hash}.indexing.o` during LTO indexing) rather than as an rlib.
        backrefs_flag.append("-Wl,--warn-backrefs-exclude=*/{package}/{target}-*".format(package = ctx.label.package, target = target))

    (linking_context, _) = cc_common.create_linking_context_from_compilation_outputs(
        name = src.basename,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        compilation_outputs = compilation_outputs,
        linking_contexts = [cc_info.linking_context],
        user_link_flags = backrefs_flag,
    )

    debug_context = cc_common.merge_debug_context([
        cc_info._debug_context if hasattr(cc_info, "_debug_context") else cc_info.debug_context(),
        cc_common.create_debug_context(compilation_outputs),
    ])

    # `compilation_context` already carries the headers and header modules of `cc_infos` as
    # non-exported deps. Don't re-export `cc_infos` here: the caller decides which module maps
    # dependents may use (see `RustBindingsFromCcInfo.cc_info`).
    return CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
        debug_context = debug_context,
    )
