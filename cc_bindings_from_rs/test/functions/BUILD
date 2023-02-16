"""End-to-end tests of `cc_bindings_from_rs`, focusing on function-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

licenses(["notice"])

rust_library(
    name = "functions",
    testonly = 1,
    srcs = ["functions.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "functions_cc_api",
    testonly = 1,
    crate = ":functions",
)

cc_test(
    name = "functions_test",
    srcs = ["functions_test.cc"],
    tags = [
        # TODO(b/269496347): Re-enable when this is fixed.
        "noguitar",
    ],
    deps = [
        ":functions_cc_api",
        "@com_google_googletest//:gtest_main",
        "//support/rstd:char",
    ],
)
