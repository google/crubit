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
    name = "matchers",
    srcs = ["matchers.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "matchers_cc_api",
    crate = ":matchers",
)

crubit_cc_test(
    name = "matchers_test",
    srcs = ["matchers_test.cc"],
    deps = [
        ":matchers_cc_api",
        "//support/rs_std",
        "@abseil-cpp//absl/strings",
        "@googletest//:gtest_main",
    ],
)
