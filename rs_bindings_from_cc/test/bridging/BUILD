load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

licenses(["notice"])

cc_library(
    name = "string_test_lib",
    hdrs = ["string_test_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support/internal:bindings_support",  # build_cleaner: keep
    ],
)

rust_api_from_cpp(
    name = "string_test_lib_rust",
    cpp_target = ":string_test_lib",
)

crubit_rust_test(
    name = "string_test",
    srcs = ["string_test.rs"],
    deps = [
        ":string_test_lib_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)
