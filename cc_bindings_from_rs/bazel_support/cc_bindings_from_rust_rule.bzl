# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""`cc_bindings_from_rust` rule.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load(
    "@rules_rust//rust:rust_common.bzl",
    "CrateInfo",
)
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")

def _generate_bindings(ctx, basename, crate_root, rustc_args):
    h_out_file = ctx.actions.declare_file(basename + "_cc_api.h")
    rs_out_file = ctx.actions.declare_file(basename + "_cc_api_impl.rs")

    crubit_args = ctx.actions.args()
    crubit_args.add("--h-out", h_out_file)
    crubit_args.add("--rs-out", rs_out_file)
    crubit_args.add("--rustfmt-exe-path", ctx.file._rustfmt)
    crubit_args.add("--rustfmt-config-path", ctx.file._rustfmt_cfg)

    ctx.actions.run(
        outputs = [h_out_file, rs_out_file],
        inputs = [crate_root, ctx.file._rustfmt, ctx.file._rustfmt_cfg],
        executable = ctx.executable._cc_bindings_from_rs_tool,
        mnemonic = "CcBindingsFromRust",
        progress_message = "Generating C++ bindings from Rust: %s" % h_out_file,
        arguments = [crubit_args, "--", rustc_args],
    )

    return (h_out_file, rs_out_file)

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

    # TODO(b/254097223): Retrieve `rs_out_file`, compile it, and include it in
    # the `linking_context`.
    (h_out_file, _) = _generate_bindings(ctx, basename, crate_root, rustc_args)

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
        linking_contexts = [ctx.attr.crate[CcInfo].linking_context],
    )
    return [CcInfo(
        compilation_context = compilation_context,
        linking_context = linking_context,
    )]

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
    fragments = ["cpp"],
)
