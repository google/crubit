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
    visibility = [
        "//:__subpackages__",
        "//cloud/vmm/tiny/vmm:__pkg__",
    ],
    deps = [
        ":cc_bindings_from_rust_cli_flag_aspect_hint_bzl",
        ":cc_bindings_from_rust_library_config_aspect_hint_bzl",
        ":providers_bzl",
        "//features:crubit_feature_hint_bzl",
        "//features:global_features_bzl",
        "//rs_bindings_from_cc/bazel_support:compile_rust_bzl",
        "//rs_bindings_from_cc/bazel_support:providers_bzl",
        "@bazel_skylib//rules:common_settings",
        "@bazel_tools//tools/cpp:toolchain_utils_bzl",
        "@rules_rust//rust:bzl_lib",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

bzl_library(
    name = "providers_bzl",
    srcs = ["providers.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
)

bzl_library(
    name = "cc_bindings_from_rust_cli_flag_aspect_hint_bzl",
    srcs = ["cc_bindings_from_rust_cli_flag_aspect_hint.bzl"],
    visibility = ["//visibility:private"],
)

bzl_library(
    name = "cc_bindings_from_rust_library_config_aspect_hint_bzl",
    srcs = ["cc_bindings_from_rust_library_config_aspect_hint.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        ":providers_bzl",
        "@bazel_skylib//lib:collections",
    ],
)

toolchain_type(
    name = "toolchain_type",
    visibility = ["//:__subpackages__"],
)
