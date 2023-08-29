"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(":deps_for_bindings_are_in_target_cfg_test.bzl", "deps_for_bindings_are_in_target_cfg_test")

package(default_applicable_licenses = ["//:license"])

deps_for_bindings_are_in_target_cfg_test(name = "deps_for_bindings_are_in_target_cfg_test")

bzl_library(
    name = "deps_for_bindings_are_in_target_cfg_test_bzl",
    srcs = ["deps_for_bindings_are_in_target_cfg_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
        "@bazel_skylib//lib:unittest",
    ],
)
