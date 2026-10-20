"""End-to-end test of no_unique_address."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "no_unique_address",
    hdrs = ["no_unique_address.h"],
    aspect_hints = ["//features:experimental"],
)

rust_api_from_cpp(
    name = "no_unique_address_rust",
    cpp_target = ":no_unique_address",
)

crubit_rust_test(
    name = "no_unique_address_test",
    srcs = ["no_unique_address_test.rs"],
    deps = [
        ":no_unique_address_rust",
        "//support:ctor",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)
