"""End-to-end tests of `cc_bindings_from_rs`, focusing on function-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//cc_bindings_from_rs:test/crubit_cc_test.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "functions",
    testonly = 1,
    srcs = ["functions.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "functions_cc_api",
    testonly = 1,
    crate = ":functions",
)

crubit_cc_test(
    name = "functions_test",
    srcs = ["functions_test.cc"],
    deps = [
        ":functions_cc_api",
        "//support/rs_std:rs_char",
        "@com_google_googletest//:gtest_main",
    ],
)
