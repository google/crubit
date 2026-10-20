load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "string_view_test_lib",
    hdrs = ["string_view_test_lib.h"],
    aspect_hints = [
        "//features:unsafe_view",
    ],
)

rust_api_from_cpp(
    name = "string_view_test_lib_rust",
    cpp_target = ":string_view_test_lib",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":string_view_test_lib_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)
