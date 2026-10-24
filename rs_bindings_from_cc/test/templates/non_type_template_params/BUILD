"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "non_type_template_params",
    hdrs = ["non_type_template_params.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "non_type_template_params_rust",
    cpp_target = ":non_type_template_params",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":non_type_template_params_rust",
        "@crate_index//:googletest",
    ],
)
