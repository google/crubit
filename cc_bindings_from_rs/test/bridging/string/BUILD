"""End-to-end tests of `cc_bindings_from_rs`, focusing on bridging std::string type."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "string",
    srcs = ["string.rs"],
    cc_deps = [
        "//support/cc_std",
    ],
)

cc_bindings_from_rust(
    name = "string_cc_api",
    crate = ":string",
)

crubit_cc_test(
    name = "string_test",
    srcs = ["string_test.cc"],
    deps = [
        ":string_cc_api",
        "@googletest//:gtest_main",
    ],
)
