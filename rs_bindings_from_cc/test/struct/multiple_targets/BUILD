"""End-to-end example using C++ libraries with dependencies."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "dependency",
    hdrs = ["dependency.h"],
)

rust_api_from_cpp(
    name = "dependency_rust",
    cpp_target = ":dependency",
)

crubit_test_cc_library(
    name = "uses_dependency",
    hdrs = ["uses_dependency.h"],
    deps = [":dependency"],
)

rust_api_from_cpp(
    name = "uses_dependency_rust",
    cpp_target = ":uses_dependency",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":dependency_rust",
        ":uses_dependency_rust",
        "@crate_index//:googletest",
    ],
)
