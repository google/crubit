# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "unique_ptr",
    srcs = ["test.rs"],
    cc_deps = [
        "//support/cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
