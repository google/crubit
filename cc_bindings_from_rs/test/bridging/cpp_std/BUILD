"""End-to-end tests of `cc_bindings_from_rs` for crates depending on `cpp_std`."""

load("@rules_rust//rust:defs.bzl", "rust_library")
load("//bazel:cpp_api_from_rust.bzl", "cpp_api_from_rust")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "cpp_std_bridging",
    srcs = ["cpp_std_bridging.rs"],
    edition = "2024",
    deps = ["//support:cpp_std"],
)

cpp_api_from_rust(
    name = "cpp_std_bridging_cc_api",
    crate = ":cpp_std_bridging",
)

crubit_cc_test(
    name = "cpp_std_bridging_test",
    srcs = ["cpp_std_bridging_test.cc"],
    deps = [
        ":cpp_std_bridging_cc_api",
        "@googletest//:gtest_main",
    ],
)
