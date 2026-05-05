"""Test for coro_wrapper attribute."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "coro_wrapper_lib",
    hdrs = ["coro_wrapper.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

crubit_rust_test(
    name = "coro_wrapper_test",
    srcs = ["test.rs"],
    cc_deps = [":coro_wrapper_lib"],
    deps = [
        "@crate_index//:googletest",
    ],
)
