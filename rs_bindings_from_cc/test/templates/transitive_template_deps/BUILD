"""Regression test for std::bitset causing Crubit to crash."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "use_template",
    hdrs = ["use_template.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "use_template_rust",
    cpp_target = ":use_template",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":use_template_rust",
        "@crate_index//:googletest",
    ],
)
