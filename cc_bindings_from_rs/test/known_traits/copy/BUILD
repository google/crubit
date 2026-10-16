"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Copy` trait"""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "copy",
    testonly = 1,
    srcs = ["copy.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "copy_cc_api",
    testonly = 1,
    crate = ":copy",
)

crubit_cc_test(
    name = "copy_test",
    srcs = ["copy_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":copy_cc_api",
        "@googletest//:gtest_main",
    ],
)
