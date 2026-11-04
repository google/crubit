"""End-to-end test of structs and unions with fixed-size C-style array fields."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "const_array_fields",
    hdrs = ["const_array_fields.h"],
)

rust_api_from_cpp(
    name = "const_array_fields_rust",
    cpp_target = ":const_array_fields",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":const_array_fields_rust",
        "@crate_index//:googletest",
    ],
)
