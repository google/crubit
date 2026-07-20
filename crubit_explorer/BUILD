# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_license//rules:license.bzl", "license")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

package(
    default_applicable_licenses = [":license"],
    default_visibility = ["//visibility:private"],
)

license(
    name = "license",
    package_name = "crubit_explorer",
)

rust_binary(
    name = "server",
    srcs = ["src/main.rs"],
    edition = "2024",
    visibility = ["//visibility:public"],
    deps = [
        "@crate_index//:axum",  # v0_8
        "@crate_index//:base64",  # v0_22
        "@crate_index//:serde",  # v1
        "@crate_index//:serde_json",  # v1
        "@crate_index//:tempfile",  # v3
        "@crate_index//:tokio",  # v1
        "@crate_index//:tower-http",
    ],
)

rust_test(
    name = "server_test",
    size = "small",
    srcs = ["src/main.rs"],
    edition = "2024",
    deps = [
        "@crate_index//:axum",  # v0_8
        "@crate_index//:base64",  # v0_22
        "@crate_index//:serde",  # v1
        "@crate_index//:serde_json",  # v1
        "@crate_index//:tempfile",  # v3
        "@crate_index//:tokio",  # v1
        "@crate_index//:tower",  # v0_5
        "@crate_index//:tower-http",
    ],
)
