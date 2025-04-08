"""End-to-end tests of `cc_bindings_from_rs`, focusing on union-related
bindings."""

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
    name = "unions",
    testonly = 1,
    srcs = ["unions.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
)

cc_bindings_from_rust(
    name = "unions_cc_api",
    testonly = 1,
    crate = ":unions",
)

crubit_cc_test(
    name = "unions_test",
    srcs = ["unions_test.cc"],
    deps = [
        ":unions_cc_api",
        "@googletest//:gtest_main",
    ],
)
