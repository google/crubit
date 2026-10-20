"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "definition_in_cc",
    srcs = ["definition_in_cc.cc"],
    hdrs = ["definition_in_cc.h"],
    aspect_hints = ["//features:experimental"],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "definition_in_cc_rust",
    cpp_target = ":definition_in_cc",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":definition_in_cc_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
