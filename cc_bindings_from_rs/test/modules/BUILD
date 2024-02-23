"""End-to-end tests of `cc_bindings_from_rs`, focusing on
module/namespace-related bindings."""

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
    name = "modules",
    testonly = 1,
    srcs = ["modules.rs"],
    deps = [
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

cc_bindings_from_rust(
    name = "modules_cc_api",
    testonly = 1,
    crate = ":modules",
)

crubit_cc_test(
    name = "modules_test",
    srcs = ["modules_test.cc"],
    deps = [
        ":modules_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
