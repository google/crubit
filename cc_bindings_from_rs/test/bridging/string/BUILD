"""End-to-end tests of `cc_bindings_from_rs`, focusing on bridging std::string type."""

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

rust_library(
    name = "string",
    srcs = ["string.rs"],
    edition = "2024",
    deps = [
        "//support/cc_std",
    ],
)

cpp_api_from_rust(
    name = "string_cc_api",
    crate = ":string",
)

crubit_cc_test(
    name = "string_test",
    srcs = ["string_test.cc"],
    # TODO(b/575815022): Re-enable once `cc_std` links on macOS.
    tags = ["no_macos"],
    deps = [
        ":string_cc_api",
        "@googletest//:gtest_main",
    ],
)
