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
    name = "tuples",
    srcs = ["tuples.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
)

golden_test(
    name = "tuples_golden_test",
    basename = "tuples",
    golden_h = "tuples_cc_api.h",
    golden_rs = "tuples_cc_api_impl.rs",
    rust_library = "tuples",
)

cc_bindings_from_rust(
    name = "tuples_cc_api",
    testonly = 1,
    crate = ":tuples",
)

crubit_cc_test(
    name = "tuples_test",
    srcs = ["tuples_test.cc"],
    deps = [
        ":tuples_cc_api",
        "@googletest//:gtest_main",
    ],
)
