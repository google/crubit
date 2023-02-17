"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load(":headers_and_targets_test.bzl", "headers_and_targets_test")
load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

headers_and_targets_test(name = "headers_and_targets_test")

bzl_library(
    name = "headers_and_targets_test_bzl",
    srcs = ["headers_and_targets_test.bzl"],
    parse_tests = False,
    visibility = ["//visibility:private"],
    deps = [
        "@bazel_skylib//lib:unittest",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)
