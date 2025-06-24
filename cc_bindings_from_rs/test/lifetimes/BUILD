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
    name = "lifetimes",
    srcs = ["lifetimes.rs"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "lifetimes_golden_test",
    basename = "lifetimes",
    golden_h = "lifetimes_cc_api.h",
    golden_rs = "lifetimes_cc_api_impl.rs",
    rust_library = "lifetimes",
)

cc_bindings_from_rust(
    name = "lifetimes_cc_api",
    testonly = 1,
    crate = ":lifetimes",
)

crubit_cc_test(
    name = "lifetimes_test",
    srcs = ["lifetimes_test.cc"],
    deps = [
        ":lifetimes_cc_api",
        "@googletest//:gtest_main",
    ],
)
