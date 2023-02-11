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

licenses(["notice"])

rust_library(
    name = "impls",
    testonly = 1,
    srcs = ["impls.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "impls_cc_api",
    testonly = 1,
    crate = ":impls",
)

cc_test(
    name = "impls_test",
    srcs = ["impls_test.cc"],
    deps = [
        ":impls_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
