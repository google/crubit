"""End-to-end test of using a simple struct."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "fields",
    hdrs = ["fields.h"],
)

rust_api_from_cpp(
    name = "fields_rust",
    cpp_target = ":fields",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":fields_rust",
        "@crate_index//:googletest",
    ],
)
