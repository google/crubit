# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Helper for constructing AdditionalRustSrcsProviderInfo."""

load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
load(
    "@rules_rust//rust/private:providers.bzl",
    "BuildInfo",
    "CrateInfo",
    "DepInfo",
    "DepVariantInfo",
)
load("@bazel_skylib//lib:collections.bzl", "collections")
load(
    "@@//rs_bindings_from_cc/bazel_support:providers.bzl",
    "AdditionalRustSrcsProviderInfo",
    "RustBindingsFromCcInfo",
)

visibility([
    "//net/proto2/compiler/stubby/cc/...",
    "//rs_bindings_from_cc/bazel_support/...",
])

def _create_dep_variant_info(dep):
    return DepVariantInfo(
        crate_info = dep[CrateInfo] if CrateInfo in dep else None,
        dep_info = dep[DepInfo] if DepInfo in dep else None,
        build_info = dep[BuildInfo] if BuildInfo in dep else None,
        cc_info = dep[CcInfo] if CcInfo in dep else None,
    )

def _get_additional_rust_deps_variant_info(deps_list):
    """Returns DepVariantInfo of `deps` associated with the `_target`.

    Args:
        deps_list: label list of deps.

    Returns:
        A list of `DepVariantInfo` of the given `deps`.
    """
    return [
        _create_dep_variant_info(dep)
        for dep in deps_list
    ]

def _get_additional_cc_deps_variant_info(cc_deps_list):
    """Returns DepVariantInfo of `cc_deps` associated with the `_target`.

    Args:
        cc_deps_list: label list of cc_deps.

    Returns:
        A list of `DepVariantInfo` of the given `cc_deps`.
    """
    additional_cc_deps = []
    for cc_dep in cc_deps_list:
        if RustBindingsFromCcInfo not in cc_dep:
            fail("cc_dep (%s) does not provide RustBindingsFromCcInfo" % cc_dep)
        if cc_dep[RustBindingsFromCcInfo].dep_variant_info:
            additional_cc_deps.extend([cc_dep[RustBindingsFromCcInfo].dep_variant_info])
    return collections.uniq(additional_cc_deps)

def make_additional_rust_srcs_provider(srcs, namespace_path, deps, cc_deps, cc_support_deps = []):
    return AdditionalRustSrcsProviderInfo(
        srcs = srcs,
        namespace_path = namespace_path,
        deps = _get_additional_rust_deps_variant_info(deps),
        cc_deps = _get_additional_cc_deps_variant_info(cc_deps),
        cc_support_deps = [
            dep[CcInfo]
            for dep in cc_support_deps
        ],
    )
