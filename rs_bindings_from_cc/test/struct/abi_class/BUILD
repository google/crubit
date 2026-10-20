"""End-to-end test of ABI classification of structs."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "abi_class",
    hdrs = ["abi_class.h"],
)

rust_api_from_cpp(
    name = "abi_class_rust",
    cpp_target = ":abi_class",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":abi_class_rust",
        "@crate_index//:googletest",
    ],
)
