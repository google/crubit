"""End-to-end tests of `cc_bindings_from_rs`, focusing on struct-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:private"],
)

rust_library(
    name = "impls",
    testonly = 1,
    srcs = ["impls.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    edition = "2024",
)

cpp_api_from_rust(
    name = "impls_cc_api",
    testonly = 1,
    crate = ":impls",
)

crubit_cc_test(
    name = "impls_test",
    srcs = ["impls_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":impls_cc_api",
        "@googletest//:gtest_main",
    ],
)
