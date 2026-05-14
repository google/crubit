load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_test")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

licenses(["notice"])

cc_library(
    name = "lib",
    srcs = ["lib.cc"],
    hdrs = ["lib.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "lib_rust",
    cpp_target = ":lib",
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)

crubit_rust_test(
    name = "crubit_test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)
