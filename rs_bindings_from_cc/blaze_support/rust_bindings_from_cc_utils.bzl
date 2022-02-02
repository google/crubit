# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

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

def compile_cc(
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

def compile_rust(ctx, attr, src, deps):
    """Compiles a Rust source file.

    Args:
      ctx: The rule context.
      attr: The current rule's attributes.
      src: The source file to be compiled.
      deps: List[DepVariantInfo]: A list of dependencies needed.

    Returns:
      A DepVariantInfo provider.
    """
    toolchain = ctx.toolchains["//third_party/bazel_rules/rules_rust/rust:toolchain"]

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
        crate_info = providers[0],
        dep_info = providers[1],
        cc_info = None,
        build_info = None,
    )

bindings_attrs = {
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
}
