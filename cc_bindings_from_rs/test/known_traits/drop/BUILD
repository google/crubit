"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Drop` trait"""

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

licenses(["notice"])

rust_library(
    name = "drop",
    testonly = 1,
    srcs = ["drop.rs"],
    deps = [
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

cc_bindings_from_rust(
    name = "drop_cc_api",
    testonly = 1,
    crate = ":drop",
)

crubit_cc_test(
    name = "drop_test",
    srcs = ["drop_test.cc"],
    deps = [
        ":drop_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
