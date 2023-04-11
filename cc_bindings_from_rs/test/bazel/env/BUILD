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
    name = "env",
    testonly = 1,
    srcs = ["env.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "env_cc_api",
    testonly = 1,
    crate = ":env",
)

cc_test(
    name = "env_test",
    srcs = ["env_test.cc"],
    deps = [
        ":env_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
