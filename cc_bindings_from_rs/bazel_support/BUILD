load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "@bazel_skylib//rules:common_settings.bzl",
    "bool_flag",
)

package(default_applicable_licenses = ["//:license"])

bool_flag(
    name = "generate_error_report",
    build_setting_default = False,
    visibility = ["//visibility:public"],
)

bzl_library(
    name = "cc_bindings_from_rust_rule_bzl",
    srcs = ["cc_bindings_from_rust_rule.bzl"],
    visibility = ["//:__subpackages__"],
)

bzl_library(
    name = "providers_bzl",
    srcs = ["providers.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
)

toolchain_type(
    name = "toolchain_type",
    visibility = ["//:__subpackages__"],
)
