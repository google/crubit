# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cc_bindings_from_rust` rule.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:providers.bzl",
    "DepInfo",
    "DepVariantInfo",
)
load(
    "@rules_rust//rust:rust_common.bzl",
    "CrateInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:compile_rust.bzl",
    "compile_rust",
)
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")

def _generate_bindings(ctx, basename, inputs, rustc_args):
    """Invokes the `cc_bindings_from_rs` tool to generate C++ bindings for a Rust crate.

    Args:
      ctx: The rule context.
      basename: The basename for the generated files
      rustc_args: `rustc` flags to pass to `cc_bindings_from_rs`.
           TODO(b/258449205): Extract `rustc_args` from the target `crate`.
      inputs: `cc_bindings_from_rs` inputs specific to the target `crate`
           TODO(b/258449205): Extract `inputs` from the target `crate`.

    Returns:
      A pair of files:
      - h_out_file (named "<basename>_cc_api.h")
      - rs_out_file (named "<basename>_cc_api_impl.rs")
    """
    h_out_file = ctx.actions.declare_file(basename + "_cc_api.h")
    rs_out_file = ctx.actions.declare_file(basename + "_cc_api_impl.rs")

    crubit_args = ctx.actions.args()
    crubit_args.add("--h-out", h_out_file)
    crubit_args.add("--rs-out", rs_out_file)
    crubit_args.add("--rustfmt-exe-path", ctx.file._rustfmt)
    crubit_args.add("--rustfmt-config-path", ctx.file._rustfmt_cfg)

    ctx.actions.run(
        outputs = [h_out_file, rs_out_file],
        inputs = inputs + [ctx.file._rustfmt, ctx.file._rustfmt_cfg],
        executable = ctx.executable._cc_bindings_from_rs_tool,
        mnemonic = "CcBindingsFromRust",
        progress_message = "Generating C++ bindings from Rust: %s" % h_out_file,
        arguments = [crubit_args, "--", rustc_args],
    )

    return (h_out_file, rs_out_file)

def _make_cc_info_for_h_out_file(ctx, h_out_file, linking_contexts):
    """Creates and returns CcInfo for the generated ..._cc_api.h header file.

    Args:
      ctx: The rule context.
      h_out_file: The generated "..._cc_api.h" header file
      linking_contexts: Linking contexts - should include both:
          1) the target `crate` and
          2) the compiled Rust glue crate (`..._cc_api_impl.rs` file).

    Returns:
      A CcInfo provider.
    """
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
    )
    (linking_context, _) = cc_common.create_linking_context_from_compilation_outputs(
        name = ctx.label.name,
        actions = ctx.actions,
        feature_configuration = feature_configuration,
        cc_toolchain = cc_toolchain,
        compilation_outputs = compilation_outputs,
        linking_contexts = linking_contexts,
    )
    return CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
    )

def _compile_rs_out_file(ctx, rs_out_file, target_crate):
    """Compiles the generated "..._cc_api_impl.rs" file.

    Args:
      ctx: The rule context.
      rs_out_file: The generated "..._cc_api_impl.rs" file
      target_crate: The value of `ctx.attr.crate`

    Returns:
      LinkingContext for linking in the generated "..._cc_api_impl.rs".
    """
    deps = [DepVariantInfo(
        crate_info = target_crate[CrateInfo],
        dep_info = target_crate[DepInfo],
        cc_info = target_crate[CcInfo],
        build_info = None,
    )]
    dep_variant_info = compile_rust(ctx, ctx.attr, rs_out_file, [], deps)
    return dep_variant_info.cc_info.linking_context

def _cc_bindings_from_rust_rule_impl(ctx):
    basename = ctx.attr.crate.label.name

    crate_root = ctx.attr.crate[CrateInfo].root

    # TODO(b/258449205): Extract `rustc_args` from the target `crate` (instead
    # of figouring out the `crate_root` and hard-coding `--crate-type`,
    # `panic=abort`, etc.).  It seems that `BuildInfo` from
    # @rules_rust//rust/private/providers.bzl is not
    # exposed publicly?
    rustc_args = ctx.actions.args()
    rustc_args.add(crate_root)
    rustc_args.add("--crate-type", "lib")
    rustc_args.add("--codegen", "panic=abort")

    (h_out_file, rs_out_file) = _generate_bindings(ctx, basename, [crate_root], rustc_args)

    impl_linking_context = _compile_rs_out_file(ctx, rs_out_file, ctx.attr.crate)

    target_cc_info = ctx.attr.crate[CcInfo]
    target_crate_linking_context = target_cc_info.linking_context
    cc_info = _make_cc_info_for_h_out_file(
        ctx,
        h_out_file,
        [target_crate_linking_context, impl_linking_context],
    )
    return [cc_info]

# TODO(b/257283134): Register actions via an `aspect`, rather than directly
# from the `rule` implementation?
cc_bindings_from_rust = rule(
    implementation = _cc_bindings_from_rust_rule_impl,
    doc = "Rule for generating C++ bindings for a Rust library.",
    attrs = {
        "crate": attr.label(
            doc = "Rust library to generate C++ bindings for",
            allow_files = False,
            mandatory = True,
            providers = [CrateInfo],
        ),
        "_cc_bindings_from_rs_tool": attr.label(
            default = Label("//cc_bindings_from_rs:cc_bindings_from_rs_legacy_toolchain_runner.sar"),
            executable = True,
            cfg = "exec",
            allow_single_file = True,
        ),
        "_cc_toolchain": attr.label(
            default = "@bazel_tools//tools/cpp:current_cc_toolchain",
        ),
        "_process_wrapper": attr.label(
            default = "@rules_rust//util/process_wrapper",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_rustfmt": attr.label(
            default = "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_rustfmt_cfg": attr.label(
            default = "//nowhere:rustfmt.toml",
            allow_single_file = True,
        ),
    },
    toolchains = [
        "@rules_rust//rust:toolchain",
    ],
    fragments = ["cpp"],
)
