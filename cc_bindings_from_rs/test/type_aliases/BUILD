"""End-to-end tests of `cc_bindings_from_rs`, focusing on type aliases bindings."""

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
    name = "type_aliases",
    testonly = 1,
    srcs = ["type_aliases.rs"],
)

cc_bindings_from_rust(
    name = "type_aliases_cc_api",
    testonly = 1,
    crate = ":type_aliases",
)

crubit_cc_test(
    name = "type_aliases_test",
    srcs = ["type_aliases_test.cc"],
    deps = [
        ":type_aliases_cc_api",
        "@googletest//:gtest_main",
    ],
)
