# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_crubit//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_binary",
    "crubit_rust_test",
)
load("@rules_license//rules:license.bzl", "license")

package(
    default_applicable_licenses = [":license"],
    default_visibility = ["//visibility:private"],
)

license(
    name = "license",
    package_name = "crubit_explorer",
)

crubit_rust_binary(
    name = "server",
    srcs = ["src/main.rs"],
    data = [
        "@rules_crubit//cc_bindings_from_rs",
        "@rules_rust//rust/toolchain:current_rustc_files",
        "@rules_rust//rust/toolchain:current_rustc_lib_files",
    ],
    edition = "2024",
    env = {
        "RUSTC_RUNFILES_PATH": "$(rlocationpath @rules_rust//rust/toolchain:current_rustc_files)",
    },
    visibility = ["//visibility:public"],
    deps = [
        "@crubit_explorer_crates//:axum",
        "@crubit_explorer_crates//:base64",
        "@crubit_explorer_crates//:serde",
        "@crubit_explorer_crates//:serde_json",
        "@crubit_explorer_crates//:tempfile",
        "@crubit_explorer_crates//:tokio",
        "@rules_rust//tools/runfiles",
    ],
)

crubit_rust_test(
    name = "server_test",
    size = "small",
    srcs = ["src/main.rs"],
    data = [
        "@rules_crubit//cc_bindings_from_rs",
        "@rules_rust//rust/toolchain:current_rustc_files",
        "@rules_rust//rust/toolchain:current_rustc_lib_files",
    ],
    edition = "2024",
    env = {
        "RUSTC_RUNFILES_PATH": "$(rlocationpath @rules_rust//rust/toolchain:current_rustc_files)",
    },
    deps = [
        "@crubit_explorer_crates//:axum",
        "@crubit_explorer_crates//:base64",
        "@crubit_explorer_crates//:serde",
        "@crubit_explorer_crates//:serde_json",
        "@crubit_explorer_crates//:tempfile",
        "@crubit_explorer_crates//:tokio",
        "@crubit_explorer_crates//:tower",
        "@rules_rust//tools/runfiles",
    ],
)
