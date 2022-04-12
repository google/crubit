"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load(
    "//rs_bindings_from_cc/bazel_support:deps_for_bindings.bzl",
    "deps_for_bindings",
)
load("//third_party/bazel_skylib:bzl_library.bzl", "bzl_library")
load("@bazel_skylib//rules:common_settings.bzl", "bool_flag")

licenses(["notice"])

bzl_library(
    name = "rust_bindings_from_cc_aspect",
    srcs = ["rust_bindings_from_cc_aspect.bzl"],
    visibility = ["//third_party/crubit:__subpackages__"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:deps_for_bindings_bzl",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
        "@rules_rust//rust:bzl_lib",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

bzl_library(
    name = "with_cc_toolchain_flags",
    srcs = ["with_cc_toolchain_flags.bzl"],
    visibility = ["//fuzzer:__pkg__"],
    deps = [
        "//tools/build_defs/cc:action_names",
        "//tools/cpp:toolchain_utils",
    ],
)

bzl_library(
    name = "toolchain_headers_bzl",
    srcs = ["toolchain_headers.bzl"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:deps_for_bindings_bzl",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_transition",
    srcs = ["rust_bindings_from_cc_transition.bzl"],
)

bzl_library(
    name = "rust_bindings_from_cc_binary_bzl",
    srcs = ["rust_bindings_from_cc_binary.bzl"],
    deps = [":rust_bindings_from_cc_transition"],
)

bzl_library(
    name = "deps_for_bindings_bzl",
    srcs = ["deps_for_bindings.bzl"],
    visibility = ["//third_party/crubit:__subpackages__"],
    deps = [
        ":rust_bindings_from_cc_transition",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_utils_bzl",
    srcs = ["rust_bindings_from_cc_utils.bzl"],
    visibility = ["//third_party/crubit:__subpackages__"],
    deps = [
        "//tools/build_defs/cc:action_names",
        "//tools/cpp:toolchain_utils",
        "@rules_rust//rust:bzl_lib",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

alias(
    name = "rust_bindings_from_cc_target",
    actual = select({
        ":use_fake_bindings_generator": ":fake_rust_bindings_from_cc",
        "//conditions:default": "//rs_bindings_from_cc:rs_bindings_from_cc",
    }),
    visibility = ["//visibility:public"],
)

config_setting(
    name = "use_fake_bindings_generator",
    flag_values = {
        ":use_actual_bindings_generator": "False",
    },
)

bool_flag(
    name = "use_actual_bindings_generator",
    build_setting_default = True,
    visibility = ["//visibility:public"],
)

sh_binary(
    name = "fake_rust_bindings_from_cc",
    srcs = ["fake_rust_bindings_from_cc.sh"],
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
