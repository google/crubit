"""End-to-end test of a C++ function declared as a `friend` in one target and redeclared in
another target."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "friend",
    hdrs = ["friend.h"],
    aspect_hints = ["//features:experimental"],
)

rust_api_from_cpp(
    name = "friend_rust",
    cpp_target = ":friend",
)

crubit_test_cc_library(
    name = "redeclaration",
    srcs = ["redeclaration.cc"],
    hdrs = ["redeclaration.h"],
    aspect_hints = ["//features:experimental"],
    deps = [":friend"],
)

rust_api_from_cpp(
    name = "redeclaration_rust",
    cpp_target = ":redeclaration",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":friend_rust",
        ":redeclaration_rust",
        "@crate_index//:googletest",
    ],
)
