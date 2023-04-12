load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "needs_feature",
    testonly = 1,
    srcs = ["needs_feature.rs"],
    crate_features = [
        "required_feature",
    ],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "needs_feature_cc_api",
    testonly = 1,
    crate = ":needs_feature",
)

cc_test(
    name = "needs_feature_test",
    srcs = ["needs_feature_test.cc"],
    deps = [
        ":needs_feature_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
