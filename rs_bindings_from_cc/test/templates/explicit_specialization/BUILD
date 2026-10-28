"""End-to-end example of explicit template specialization working."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "explicit_specialization",
    hdrs = ["explicit_specialization.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "explicit_specialization_rust",
    cpp_target = ":explicit_specialization",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":explicit_specialization_rust",
        "@crate_index//:googletest",
    ],
)
