"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Clone` trait"""

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
    # `name = "clone"` would make the bindings conflict with a top-level item in the C++ standard library
    # See also b/287222576.
    name = "rs_clone",
    testonly = 1,
    srcs = ["clone.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "rs_clone_cc_api",
    testonly = 1,
    crate = ":rs_clone",
)

crubit_cc_test(
    name = "clone_test",
    srcs = ["clone_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":rs_clone_cc_api",
        "@googletest//:gtest_main",
    ],
)
