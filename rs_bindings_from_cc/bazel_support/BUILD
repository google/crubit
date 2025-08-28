load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "@bazel_skylib//rules:common_settings.bzl",
    "bool_flag",
)
load(
    "//rs_bindings_from_cc/bazel_support:deps_for_bindings.bzl",
    "deps_for_bindings",
)

package(default_applicable_licenses = ["//:license"])

bzl_library(
    name = "rust_bindings_from_cc_aspect",
    srcs = ["rust_bindings_from_cc_aspect.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_cli_flag_aspect_hint",
        ":rust_bindings_from_cc_utils_bzl",
        "//features:crubit_feature_hint_bzl",
        "@bazel_skylib//lib:collections",
        "@protobuf//rust/bazel:aspects_bzl",
        "@rules_cc//cc/common",
        "@rules_rust//allowlists:bzl_lib",
    ],
)

bzl_library(
    name = "toolchain_headers_bzl",
    srcs = ["toolchain_headers.bzl"],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_utils_bzl",
        "@rules_cc//cc/common",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_transition",
    srcs = ["rust_bindings_from_cc_transition.bzl"],
)

bzl_library(
    name = "rust_bindings_from_cc_binary_bzl",
    srcs = ["rust_bindings_from_cc_binary.bzl"],
    visibility = ["//visibility:private"],
    deps = [":rust_bindings_from_cc_transition"],
)

bzl_library(
    name = "deps_for_bindings_bzl",
    srcs = ["deps_for_bindings.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_transition",
        "@rules_cc//cc/common",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_utils_bzl",
    srcs = ["rust_bindings_from_cc_utils.bzl"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":compile_cc_bzl",
        ":compile_rust_bzl",
        ":generate_bindings_bzl",
        ":providers_bzl",
        "//cc_bindings_from_rs/bazel_support:providers_bzl",
        "@bazel_tools//tools/cpp:toolchain_utils",
        "@rules_cc//cc/common",
    ],
)

bzl_library(
    name = "providers_bzl",
    srcs = ["providers.bzl"],
    visibility = [
    ],
)

bzl_library(
    name = "compile_cc_bzl",
    srcs = ["compile_cc.bzl"],
    deps = ["@rules_cc//cc/common"],
)

bzl_library(
    name = "generate_bindings_bzl",
    srcs = ["generate_bindings.bzl"],
    deps = [
        "@bazel_skylib//rules:common_settings",
        "@bazel_tools//tools/build_defs/cc:action_names",
        "@rules_cc//cc/common",
    ],
)

bzl_library(
    name = "compile_rust_bzl",
    srcs = ["compile_rust.bzl"],
    visibility = ["//cc_bindings_from_rs/bazel_support:__pkg__"],
    deps = [
        "@bazel_skylib//lib:structs",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_cli_flag_aspect_hint",
    srcs = ["rust_bindings_from_cc_cli_flag_aspect_hint.bzl"],
)

bzl_library(
    name = "additional_rust_srcs_for_crubit_bindings_aspect_hint_bzl",
    srcs = ["additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl"],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_aspect",
        "@bazel_skylib//lib:collections",
        "@rules_cc//cc/common",
        "@rules_rust//rust/private:providers_bzl",
    ],
)

bool_flag(
    name = "use_actual_bindings_generator",
    build_setting_default = True,
    visibility = ["//visibility:public"],
)

filegroup(
    name = "fake_rust_bindings_from_cc",
    srcs = ["fake_rust_bindings_from_cc.sh"],
    visibility = ["//:__subpackages__"],
)

alias(
    name = "deps_for_bindings",
    actual = select({
        ":remove_deps": ":empty_deps",
        "//conditions:default": "//rs_bindings_from_cc:deps_for_bindings",
    }),
    visibility = ["//visibility:public"],
)

config_setting(
    name = "remove_deps",
    flag_values = {
        ":use_actual_deps": "False",
    },
)

bool_flag(
    name = "use_actual_deps",
    build_setting_default = True,
    visibility = ["//visibility:public"],
)

deps_for_bindings(
    name = "empty_deps",
)

bool_flag(
    name = "generate_error_report",
    build_setting_default = False,
    visibility = ["//visibility:public"],
)

toolchain_type(
    name = "toolchain_type",
    visibility = ["//:__subpackages__"],
)
