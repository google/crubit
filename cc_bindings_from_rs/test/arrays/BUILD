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
    name = "arrays",
    srcs = ["arrays.rs"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "arrays_golden_test",
    basename = "arrays",
    golden_h = "arrays_cc_api.h",
    golden_rs = "arrays_cc_api_impl.rs",
    rust_library = "arrays",
)

cc_bindings_from_rust(
    name = "arrays_cc_api",
    testonly = 1,
    crate = ":arrays",
)

crubit_cc_test(
    name = "arrays_test",
    srcs = ["arrays_test.cc"],
    deps = [
        ":arrays_cc_api",
        "@googletest//:gtest_main",
    ],
)
