"""Test for coro_wrapper attribute."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "coro_wrapper_lib",
    hdrs = ["coro_wrapper.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "coro_wrapper_lib_rust",
    cpp_target = ":coro_wrapper_lib",
)

crubit_rust_test(
    name = "coro_wrapper_test",
    srcs = ["test.rs"],
    deps = [
        ":coro_wrapper_lib_rust",
        "@crate_index//:googletest",
    ],
)
