"""End-to-end tests of `cc_bindings_from_rs`, focusing on
module/namespace-related bindings."""

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
    name = "modules",
    testonly = 1,
    srcs = ["modules.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "modules_cc_api",
    testonly = 1,
    crate = ":modules",
)

cc_test(
    name = "modules_test",
    srcs = ["modules_test.cc"],
    tags = [
        # TODO(b/269496347): Remove all these tags when this is fixed
        "noguitar",
        "notap",
        "manual",
    ],
    deps = [
        ":modules_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
