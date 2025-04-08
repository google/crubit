"""End-to-end tests of `cc_bindings_from_rs`, focusing on enums-related
bindings."""

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
    name = "enums",
    testonly = 1,
    srcs = ["enums.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust(
    name = "enums_cc_api",
    testonly = 1,
    crate = ":enums",
)

crubit_cc_test(
    name = "enums_test",
    srcs = ["enums_test.cc"],
    deps = [
        ":enums_cc_api",
        "@googletest//:gtest_main",
    ],
)
