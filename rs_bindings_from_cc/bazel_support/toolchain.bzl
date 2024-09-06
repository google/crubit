# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Crubit (rs_bindings_from_cc) toolchain."""

load(
    "@@//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcToolchainInfo",
)

def _rs_bindings_from_cc_toolchain_impl(ctx):
    return [
        platform_common.ToolchainInfo(
            rs_bindings_from_cc_toolchain_info = RustBindingsFromCcToolchainInfo(
                binary = ctx.file.binary,
                builtin_headers = ctx.files.builtin_headers,
                is_on_demand = ctx.attr.is_on_demand,
            ),
        ),
    ]

rs_bindings_from_cc_toolchain = rule(
    implementation = _rs_bindings_from_cc_toolchain_impl,
    attrs = {
        "binary": attr.label(
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "builtin_headers": attr.label_list(allow_files = True),
        "is_on_demand": attr.bool(),
    },
)
