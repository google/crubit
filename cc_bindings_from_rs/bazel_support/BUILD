load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//:license"])

bzl_library(
    name = "cc_bindings_from_rust_rule_bzl",
    srcs = ["cc_bindings_from_rust_rule.bzl"],
    visibility = ["//visibility:private"],
)

bzl_library(
    name = "providers_bzl",
    srcs = ["providers.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
)
