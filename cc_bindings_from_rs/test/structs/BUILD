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
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "structs",
    testonly = 1,
    srcs = ["structs.rs"],
    deps = [
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

cc_bindings_from_rust(
    name = "structs_cc_api",
    testonly = 1,
    crate = ":structs",
)

crubit_cc_test(
    name = "structs_test",
    srcs = ["structs_test.cc"],
    deps = [
        ":structs_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
