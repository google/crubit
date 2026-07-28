# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_crubit//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_binary",
    "crubit_rust_test",
)
load("@rules_license//rules:license.bzl", "license")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

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
    srcs = [
        "src/api.rs",
        "src/main.rs",
    ],
    crate_root = "src/main.rs",
    data = [
        "//frontend:build",
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
    srcs = [
        "src/api.rs",
        "src/main.rs",
    ],
    crate_root = "src/main.rs",
    data = [
        "//frontend:build",
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
        "@crubit_explorer_crates//:googletest",
        "@crubit_explorer_crates//:serde",
        "@crubit_explorer_crates//:serde_json",
        "@crubit_explorer_crates//:tempfile",
        "@crubit_explorer_crates//:tokio",
        "@crubit_explorer_crates//:tower",
        "@rules_rust//tools/runfiles",
    ],
)

pkg_tar(
    name = "server_layer",
    srcs = [
        ":server",
        "@rules_crubit//cc_bindings_from_rs",
    ],
    package_dir = "/",
)

pkg_tar(
    name = "frontend_layer",
    srcs = ["//frontend:build"],
    package_dir = "/frontend/dist/frontend",
    strip_prefix = "frontend/dist/frontend",
)

pkg_tar(
    name = "rust_layer",
    srcs = [
        "@rules_rust//rust/toolchain:current_rustc_lib_files",
    ],
    package_dir = "/",
    strip_prefix = "external/rules_crubit++crubit_toolchains+rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain",
)

oci_image(
    name = "image",
    base = "@distroless_cc",
    entrypoint = ["/server"],
    env = {
        "RUST_LOG": "info",
    },
    tars = [
        ":server_layer",
        ":frontend_layer",
        ":rust_layer",
    ],
)

oci_load(
    name = "server_load",
    image = ":image",
    repo_tags = ["crubit_explorer:latest"],
)

filegroup(
    name = "server_image",
    srcs = [":server_load"],
    output_group = "tarball",
)
