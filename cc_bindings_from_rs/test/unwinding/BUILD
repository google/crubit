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
    name = "panic_function",
    testonly = 1,
    srcs = ["panic_function.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "panic_function_cc_api",
    testonly = 1,
    crate = ":panic_function",
)

crubit_cc_test(
    name = "unwinding_test",
    srcs = ["unwinding_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":panic_function_cc_api",
        "@googletest//:gtest_main",
    ],
)
