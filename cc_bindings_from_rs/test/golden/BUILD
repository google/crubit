load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//:license"])

bzl_library(
    name = "golden_test",
    srcs = ["golden_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule_bzl",
    ],
)
