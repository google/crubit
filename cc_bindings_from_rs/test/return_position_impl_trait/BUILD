# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "return_position_impl_trait",
    testonly = 1,
    srcs = ["return_position_impl_trait.rs"],
    aspect_hints = [
        "//features:supported",
    ],
)

cc_bindings_from_rust(
    name = "return_position_impl_trait_cc_api",
    testonly = 1,
    crate = ":return_position_impl_trait",
)

golden_test(
    name = "return_position_impl_trait_golden_test",
    basename = "return_position_impl_trait",
    golden_h = "return_position_impl_trait_cc_api.h",
    golden_rs = "return_position_impl_trait_cc_api_impl.rs",
    platforms = ["android"],
    rust_library = "return_position_impl_trait",
)
