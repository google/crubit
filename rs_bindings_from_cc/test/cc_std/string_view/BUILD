# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_test")

cc_library(
    name = "string_view_apis",
    hdrs = ["string_view_apis.h"],
)

rust_test(
    name = "string_view",
    srcs = ["test.rs"],
    cc_deps = [
        ":string_view_apis",
    ],
)
