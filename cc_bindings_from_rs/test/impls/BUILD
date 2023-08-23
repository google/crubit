"""End-to-end tests of `cc_bindings_from_rs`, focusing on struct-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", crubit_cc_test = "crubit_on_demand_cc_test")

rust_library(
    name = "impls",
    testonly = 1,
    srcs = ["impls.rs"],
)

cc_bindings_from_rust(
    name = "impls_cc_api",
    testonly = 1,
    crate = ":impls",
)

crubit_cc_test(
    name = "impls_test",
    srcs = ["impls_test.cc"],
    deps = [
        ":impls_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
