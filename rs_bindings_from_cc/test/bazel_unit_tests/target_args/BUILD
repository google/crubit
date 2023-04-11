"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load(":target_args_test.bzl", "target_args_test")
load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

target_args_test(name = "target_args_test")

bzl_library(
    name = "target_args_test_bzl",
    srcs = ["target_args_test.bzl"],
    parse_tests = False,
    visibility = ["//visibility:private"],
    deps = [
        "@bazel_skylib//lib:unittest",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)
