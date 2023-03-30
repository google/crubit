"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(":additional_rust_srcs_for_crubit_bindings_aspect_hint_test.bzl", "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

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

rust_test(
    name = "additional_rust_srcs_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":empty_cc_lib_with_additional_rust_srcs",
    ],
)

additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite(
    name = "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite",
)
