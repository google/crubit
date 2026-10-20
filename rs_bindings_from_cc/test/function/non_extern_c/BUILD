"""End-to-end example of using simple functions."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "simple_functions",
    srcs = ["simple_functions.cc"],
    hdrs = ["simple_functions.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "simple_functions_rust",
    cpp_target = ":simple_functions",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":simple_functions_rust",
        "@crate_index//:googletest",
    ],
)
