"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load(":rust_bindings_from_cc_aspect_test.bzl", "rust_bindings_from_cc_aspect_test")
load("//tools/build_defs/testing:bzl_library.bzl", "bzl_library")

licenses(["notice"])

rust_bindings_from_cc_aspect_test(name = "rust_bindings_from_cc_aspect_test")

bzl_library(
    name = "rust_bindings_from_cc_aspect_test_bzl",
    srcs = ["rust_bindings_from_cc_aspect_test.bzl"],
    parse_tests = False,
    visibility = ["//visibility:private"],
    deps = [
        "//third_party/bazel_skylib/lib:unittest",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)
