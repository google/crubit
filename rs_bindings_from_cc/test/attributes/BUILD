load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

licenses(["notice"])

cc_library(
    name = "annotalysis",
    hdrs = ["annotalysis.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/base:core_headers",
    ],
)

rust_api_from_cpp(
    name = "annotalysis_rust",
    cpp_target = ":annotalysis",
)

crubit_rust_test(
    name = "annotalysis_test",
    srcs = ["annotalysis_test.rs"],
    deps = [
        ":annotalysis_rust",
        "@crate_index//:googletest",
    ],
)
