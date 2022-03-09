# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A wrapper rule around the bindings binary.

We need this wrapper so we can specify a transition and thus avoid the circular dependency
that happens when we try to build the rs_bindings_from_cc:
rust_bindings_from_cc_aspect -> rs_bindings_from_cc -> rust_library
              ^_____________________________________________|

Disclaimer: This project is experimental, under heavy development, and should
be used yet.
"""

load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_transition.bzl",
    "rust_bindings_from_cc_transition",
)

# buildifier: disable=bzl-visibility
load(
    "//third_party/bazel_rules/rules_rust/rust/private:providers.bzl",
    "CrateInfo",
    "DepInfo",
    "DepVariantInfo",
)

GeneratedFilesDepsInfo = provider(
    doc = """A provider that serves to pass on dependencies needed when compiling the generated
          Rust and C++ files.""",
    fields = {
        "deps_for_rs_file": "list[DepVariantInfo]",
        "deps_for_cc_file": "list[CcInfo]",
    },
)

def _rust_bindings_from_cc_binary_impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.symlink(output = output, target_file = ctx.executable.binary, is_executable = True)
    dep_variant_infos = [
        DepVariantInfo(
            crate_info = dep[CrateInfo] if CrateInfo in dep else None,
            dep_info = dep[DepInfo] if DepInfo in dep else None,
            cc_info = dep[CcInfo] if CcInfo in dep else None,
            build_info = None,
        )
        for dep in ctx.attr.deps_for_generated_rs_file
    ]

    return [
        DefaultInfo(
            executable = output,
            runfiles = ctx.runfiles(
                files = [ctx.file.binary, ctx.executable._rustfmt] + ctx.files._rustfmt_cfg,
            ),
        ),
        GeneratedFilesDepsInfo(
            deps_for_rs_file = dep_variant_infos,
            deps_for_cc_file = [dep[CcInfo] for dep in ctx.attr.deps_for_generated_cc_file],
        ),
    ]

rust_bindings_from_cc_binary = rule(
    attrs = {
        "binary": attr.label(
            executable = True,
            allow_single_file = True,
            cfg = rust_bindings_from_cc_transition,
            doc = ("Executable StandaloneClangTool binary that generates Rust bindings from " +
                   "C++ code."),
        ),
        "deps_for_generated_rs_file": attr.label_list(
            cfg = rust_bindings_from_cc_transition,
            doc = "Rust dependencies that are needed to compile the generated _impl.rs file.",
            default = [],
        ),
        "deps_for_generated_cc_file": attr.label_list(
            doc = "C++ dependencies that are needed to compile the generated .cc file.",
            default = [],
        ),
        "_allowlist_function_transition": attr.label(
            default = "//tools/allowlists/function_transition_allowlist",
        ),
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
    },
    executable = True,
    implementation = _rust_bindings_from_cc_binary_impl,
)
