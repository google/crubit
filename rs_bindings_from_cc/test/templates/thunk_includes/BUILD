"""Thunks that name standard library types from implementation headers."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "uses_std_string",
    hdrs = ["uses_std_string.h"],
    aspect_hints = ["//features:template_instantiation"],
)

rust_api_from_cpp(
    name = "uses_std_string_rust",
    cpp_target = ":uses_std_string",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":uses_std_string_rust",
        "@crate_index//:googletest",
    ],
)
