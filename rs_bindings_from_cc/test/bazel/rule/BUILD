load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

cc_library(
    name = "lib",
    hdrs = ["lib.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "lib_rust",
    cpp_target = ":lib",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)
