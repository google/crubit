"""End-to-end example of using fully-instantiated templates as function return types."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "struct_fields",
    hdrs = ["struct_fields.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "struct_fields_rust",
    cpp_target = ":struct_fields",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":struct_fields_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
