load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "panic_function",
    testonly = 1,
    srcs = ["panic_function.rs"],
)

cc_bindings_from_rust(
    name = "panic_function_cc_api",
    testonly = 1,
    crate = ":panic_function",
)

crubit_cc_test(
    name = "unwinding_test",
    srcs = ["unwinding_test.cc"],
    deps = [
        ":panic_function_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
