load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    srcs = [
        "forward_declaration.cc",
    ],
    hdrs = ["forward_declaration.h"],
    aspect_hints = ["//features:experimental"],
)

rust_api_from_cpp(
    name = "forward_declaration_rust",
    cpp_target = ":forward_declaration",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":forward_declaration_rust",
        "@crate_index//:googletest",
    ],
)
