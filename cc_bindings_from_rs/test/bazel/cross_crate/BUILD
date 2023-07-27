"""End-to-end tests of `cc_bindings_from_rs`, focusing on cross-crate bindings
(bindings for items that refer to types from another crate)."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//cc_bindings_from_rs:test/crubit_cc_test.bzl", "crubit_cc_test")

rust_library(
    name = "other_crate",
    testonly = 1,
    srcs = ["other_crate.rs"],
)

rust_library(
    name = "test_api",
    testonly = 1,
    srcs = ["test_api.rs"],
    deps = [":other_crate"],
)

cc_bindings_from_rust(
    name = "test_api_cc_api",
    testonly = 1,
    crate = ":test_api",
)

crubit_cc_test(
    name = "cross_crate_test",
    srcs = ["cross_crate_test.cc"],
    deps = [
        ":test_api_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
