"""End-to-end example of using fully-instantiated templates as function return types."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "func_return_and_param_types",
    srcs = ["func_return_and_param_types.cc"],
    hdrs = ["func_return_and_param_types.h"],
    aspect_hints = ["//features:experimental"],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "func_return_and_param_types_rust",
    cpp_target = ":func_return_and_param_types",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":func_return_and_param_types_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
