"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Default` trait"""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", crubit_cc_test = "crubit_on_demand_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    # `default` is a reserved C++ keyword, so using `rs_` prefix.
    name = "rs_default",
    testonly = 1,
    srcs = ["default.rs"],
)

cc_bindings_from_rust(
    name = "rs_default_cc_api",
    testonly = 1,
    crate = ":rs_default",
)

crubit_cc_test(
    name = "default_test",
    srcs = ["default_test.cc"],
    deps = [
        ":rs_default_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
