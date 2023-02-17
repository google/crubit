"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load(":cc_std_test.bzl", "cc_std_test")
load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "foo",
    hdrs = ["foo.h"],
    deps = [":bar"],
)

cc_library(
    name = "bar",
    hdrs = ["bar.h"],
)

cc_std_test(
    name = "library_without_deps_test",
    dep = ":bar",
)

cc_std_test(
    name = "library_with_deps_test",
    dep = ":foo",
)

sh_binary(
    name = "check_include_directives",
    srcs = ["check_include_directives.sh"],
)

bzl_library(
    name = "cc_std_test_bzl",
    srcs = ["cc_std_test.bzl"],
    parse_tests = False,
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)
