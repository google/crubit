"""End-to-end test of packed structs."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "packed",
    hdrs = ["packed.h"],
    aspect_hints = ["//features:record_impl_debug"],
    deps = ["@abseil-cpp//absl/base:core_headers"],
)

rust_api_from_cpp(
    name = "packed_rust",
    cpp_target = ":packed",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":packed_rust",
        "@crate_index//:googletest",
    ],
)
