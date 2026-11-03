load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_bindings_from_rust_library_config(
    name = "rs_namespace_config",
    namespace = "test_ns::rs",
)

rust_library(
    name = "rs_namespace",
    srcs = ["rs_namespace.rs"],
    aspect_hints = [":rs_namespace_config"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "rs_namespace_cc_api",
    crate = ":rs_namespace",
)

crubit_cc_test(
    name = "rs_namespace_test",
    srcs = ["rs_namespace_test.cc"],
    deps = [
        ":rs_namespace_cc_api",
        "//support/rs_std",
        "@googletest//:gtest_main",
    ],
)
