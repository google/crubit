"""End-to-end tests of `cc_bindings_from_rs`, focusing on correct move semantics."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "move",
    srcs = ["move.rs"],
)

golden_test(
    name = "move_golden_test",
    basename = "move",
    golden_h = "move_cc_api.h",
    golden_rs = "move_cc_api_impl.rs",
    rust_library = "move",
)

cc_bindings_from_rust(
    name = "move_cc_api",
    crate = ":move",
)

crubit_cc_test(
    name = "move_test",
    srcs = ["move_test.cc"],
    deps = [
        ":move_cc_api",
        "@googletest//:gtest_main",
    ],
)
