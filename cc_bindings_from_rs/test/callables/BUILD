"""End-to-end tests of `cc_bindings_from_rs`, focusing on callable parameter bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "callables",
    srcs = ["callables.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "callables_golden_test",
    basename = "callables",
    golden_h = "callables_cc_api.h",
    golden_rs = "callables_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "callables",
)

cc_bindings_from_rust(
    name = "callables_cc_api",
    testonly = 1,
    crate = ":callables",
)

crubit_cc_test(
    name = "callables_test",
    srcs = ["callables_test.cc"],
    deps = [
        ":callables_cc_api",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)
