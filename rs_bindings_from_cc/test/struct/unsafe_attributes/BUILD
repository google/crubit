"""End-to-end example of using unsafe attributes on structs."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "unsafe_attributes",
    srcs = ["unsafe_attributes.cc"],
    hdrs = ["unsafe_attributes.h"],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "unsafe_attributes_rust",
    cpp_target = ":unsafe_attributes",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":unsafe_attributes_rust",
        "@crate_index//:googletest",
    ],
)
