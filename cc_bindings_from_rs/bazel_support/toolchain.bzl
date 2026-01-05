# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Crubit (cc_bindings_from_rs) toolchain."""

load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    "CcBindingsFromRustToolchainInfo",
)

def _cc_bindings_from_rs_toolchain_impl(ctx):
    return [
        platform_common.ToolchainInfo(
            cc_bindings_from_rs_toolchain_info = CcBindingsFromRustToolchainInfo(
                binary = ctx.executable.binary,
            ),
        ),
    ]

cc_bindings_from_rs_toolchain = rule(
    implementation = _cc_bindings_from_rs_toolchain_impl,
    attrs = {
        "binary": attr.label(
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
    },
)
