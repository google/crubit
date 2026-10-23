# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//features:crubit_feature_hint.bzl", "crubit_feature_denylists")
load("//features:global_features.bzl", "SUPPORTED_FEATURES")
load(":denylist_dir.bzl", "denylist_paths", "merge_denylists")
load(":supported_feature_denylists.bzl", "SUPPORTED_FEATURE_DENYLISTS")

package(default_applicable_licenses = ["//:license"])

# Combines the inline denylists in `supported_feature_denylists.bzl` with the per-entry files in
# `internal/<feature>/`. See `denylist_dir.bzl`.
crubit_feature_denylists(
    name = "supported_feature_denylists",
    denylists = merge_denylists(
        SUPPORTED_FEATURE_DENYLISTS,
        denylist_paths(),
        SUPPORTED_FEATURES,
    ),
    visibility = ["//visibility:public"],
)
