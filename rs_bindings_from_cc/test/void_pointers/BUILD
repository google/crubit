"""End-to-end example of using type aliases."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "void_pointers",
    hdrs = ["void_pointers.h"],
)

rust_api_from_cpp(
    name = "void_pointers_rust",
    cpp_target = ":void_pointers",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":void_pointers_rust",
        "@crate_index//:googletest",
    ],
)
