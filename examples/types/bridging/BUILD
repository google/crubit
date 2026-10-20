load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

licenses(["notice"])

cc_library(
    name = "either",
    srcs = ["either_internal.h"],
    hdrs = ["either.h"],
    aspect_hints = [
        ":additional_either_src",
        "//features:supported",
    ],
    deps = [
        "//support:annotations",  # build_cleaner: keep
        "//support:bridge_cpp",  # build_cleaner: keep
    ],
)

rust_api_from_cpp(
    name = "either_rust",
    cpp_target = ":either",
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_either_src",
    srcs = ["additional_either_src.rs"],
    namespace_path = "either",
)

cc_library(
    name = "int_or_bool",
    srcs = ["int_or_bool.cc"],
    hdrs = ["int_or_bool.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [":either"],
)

rust_api_from_cpp(
    name = "int_or_bool_rust",
    cpp_target = ":int_or_bool",
)

crubit_rust_test(
    name = "rust_int_or_bool_test",
    srcs = ["rust_int_or_bool_test.rs"],
    deps = [
        ":either_rust",
        ":int_or_bool_rust",
        "@crate_index//:googletest",
    ],
)
