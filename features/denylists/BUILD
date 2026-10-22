# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//features:crubit_feature_hint.bzl", "crubit_feature_denylists")
load(":supported_feature_denylists.bzl", "SUPPORTED_FEATURE_DENYLISTS")

package(default_applicable_licenses = ["//:license"])

crubit_feature_denylists(
    name = "supported_feature_denylists",
    denylists = SUPPORTED_FEATURE_DENYLISTS,
    visibility = ["//visibility:public"],
)
