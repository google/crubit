"""End-to-end test of structs with explicitly aligned fields."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "aligned_fields",
    hdrs = ["aligned_fields.h"],
)

rust_api_from_cpp(
    name = "aligned_fields_rust",
    cpp_target = ":aligned_fields",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":aligned_fields_rust",
        "@crate_index//:googletest",
    ],
)
