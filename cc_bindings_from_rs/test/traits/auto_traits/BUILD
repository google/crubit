# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "send_sync_types",
    testonly = 1,
    srcs = ["send_sync_types.rs"],
)

golden_test(
    name = "send_sync_types_golden_test",
    basename = "send_sync_types",
    golden_h = "send_sync_types_cc_api.h",
    golden_rs = "send_sync_types_cc_api_impl.rs",
    rust_library = "send_sync_types",
)
