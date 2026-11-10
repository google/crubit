load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

licenses(["notice"])

cc_library(
    name = "absl_hashables",
    hdrs = ["absl_hashables.h"],
    aspect_hints = ["//features:absl_hash_value"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "absl_hashables_rust",
    cpp_target = ":absl_hashables",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":absl_hashables_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
