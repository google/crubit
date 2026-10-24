load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "nested_forward_decl",
    hdrs = ["nested_forward_decl.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "nested_forward_decl_rust",
    cpp_target = ":nested_forward_decl",
)

crubit_rust_test(
    name = "test_inner_exists",
    srcs = ["test_inner_exists.rs"],
    deps = [
        ":nested_forward_decl_rust",
        "@crate_index//:googletest",
    ],
)
