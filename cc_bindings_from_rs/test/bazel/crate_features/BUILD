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
    name = "needs_feature",
    testonly = 1,
    srcs = ["needs_feature.rs"],
    crate_features = [
        "required_feature",
    ],
)

cc_bindings_from_rust(
    name = "needs_feature_cc_api",
    testonly = 1,
    crate = ":needs_feature",
)

crubit_cc_test(
    name = "needs_feature_test",
    srcs = ["needs_feature_test.cc"],
    deps = [
        ":needs_feature_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
