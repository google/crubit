"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(":additional_rust_srcs_for_crubit_bindings_aspect_hint_test.bzl", "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite")

additional_rust_srcs_for_crubit_bindings(
    name = "additional_rust_srcs_test_stub",
    srcs = [
        "stub.rs",
    ],
)

crubit_test_cc_library(
    name = "empty_cc_lib_with_additional_rust_srcs",
    hdrs = ["empty.h"],
    aspect_hints = [
        ":additional_rust_srcs_test_stub",
    ],
)

crubit_rust_test(
    name = "additional_rust_srcs_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":empty_cc_lib_with_additional_rust_srcs",
    ],
)

additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite(
    name = "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite",
)
