"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "two_template_parameters",
    hdrs = ["two_template_parameters.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "two_template_parameters_rust",
    cpp_target = ":two_template_parameters",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":two_template_parameters_rust",
        "@crate_index//:googletest",
    ],
)
