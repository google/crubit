"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "out_of_line_definition",
    hdrs = ["out_of_line_definition.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "out_of_line_definition_rust",
    cpp_target = ":out_of_line_definition",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":out_of_line_definition_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
