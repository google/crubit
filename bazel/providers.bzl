# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Re-exports Crubit providers."""

load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    _CcBindingsFromRustInfo = "CcBindingsFromRustInfo",
    _CcBindingsFromRustToolchainInfo = "CcBindingsFromRustToolchainInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    _AdditionalRustSrcsProviderInfo = "AdditionalRustSrcsProviderInfo",
)

AdditionalRustSrcsProviderInfo = _AdditionalRustSrcsProviderInfo
DepsForBindingsInfo = _DepsForBindingsInfo
GeneratedBindingsInfo = _GeneratedBindingsInfo
RustBindingsFromCcInfo = _RustBindingsFromCcInfo
RustBindingsFromCcToolchainInfo = _RustBindingsFromCcToolchainInfo
CcBindingsFromRustInfo = _CcBindingsFromRustInfo
CcBindingsFromRustToolchainInfo = _CcBindingsFromRustToolchainInfo
