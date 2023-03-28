# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Temporary feature flag to detect legacy Rust toolchain."""
# TODO(b/262583967): Remove this file when the legacy toolchain no longer exists.

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:utils.bzl",
    "find_toolchain",
)

def _impl(ctx):
    toolchain = find_toolchain(ctx)
    return [config_common.FeatureFlagInfo(value = str("third_party/unsupported_toolchains/rust" in toolchain.rustc.path))]

legacy_rust_toolchain_detector = rule(
    implementation = _impl,
    toolchains = [
        "@rules_rust//rust:toolchain",
    ],
)
