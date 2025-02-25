load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(":generation_test.bzl", "generation_test")

package(default_applicable_licenses = ["//:license"])

generation_test(name = "generation_test")

bzl_library(
    name = "generation_test_bzl",
    srcs = ["generation_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
        "@bazel_skylib//lib:unittest",
    ],
)
