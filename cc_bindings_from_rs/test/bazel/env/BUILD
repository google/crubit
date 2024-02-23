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
    name = "env",
    testonly = 1,
    srcs = ["env.rs"],
)

cc_bindings_from_rust(
    name = "env_cc_api",
    testonly = 1,
    crate = ":env",
)

crubit_cc_test(
    name = "env_test",
    srcs = ["env_test.cc"],
    deps = [
        ":env_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
