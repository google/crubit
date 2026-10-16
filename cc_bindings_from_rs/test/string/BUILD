"""End-to-end tests of `cc_bindings_from_rs`, focusing on Rust String bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "string",
    srcs = ["string.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "string_cc_api",
    crate = ":string",
)

crubit_cc_test(
    name = "string_test",
    srcs = ["string_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":string_cc_api",
        "//support/rs_std",
        "//support/rs_std:rs_alloc",
        "@googletest//:gtest_main",
    ],
)
