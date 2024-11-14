"""End-to-end tests of `cc_bindings_from_rs`, focusing on type bridging."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "rust_type",
    srcs = [
        "rust_type.rs",
    ],
    aspect_hints = [
        ":cc_type_hint",
    ],
)

cc_bindings_from_rust_library_config(
    name = "cc_type_hint",
    extra_cc_hdrs = ["cc_type.h"],
)

cc_bindings_from_rust(
    name = "rust_type_cc_api",
    crate = ":rust_type",
)

crubit_cc_test(
    name = "bridging_test",
    srcs = ["bridging_test.cc"],
    deps = [
        ":rust_type_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
