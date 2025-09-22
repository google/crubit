"""End-to-end tests of `cc_bindings_from_rs`, focusing on type composable bridging."""

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
    name = "composable_bridging",
    srcs = [
        "composable_bridging.rs",
    ],
    cc_deps = [
        "//support/cc_std",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

cc_bindings_from_rust(
    name = "composable_bridging_cc_api",
    crate = ":composable_bridging",
)

crubit_cc_test(
    name = "composable_bridging_test",
    srcs = ["composable_bridging_test.cc"],
    deps = [
        ":composable_bridging_cc_api",
        "//support/rs_std:slice_ref",
        "@googletest//:gtest_main",
    ],
)
