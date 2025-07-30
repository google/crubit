load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(":target_args_test.bzl", "target_args_test")

package(default_applicable_licenses = ["//:license"])

target_args_test(name = "target_args_test")

bzl_library(
    name = "target_args_test_bzl",
    srcs = ["target_args_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:providers_bzl",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
        "@bazel_skylib//lib:unittest",
        "@rules_cc//cc:core_rules",
    ],
)
