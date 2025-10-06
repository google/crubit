"""End-to-end tests of `cc_bindings_from_rs`, focusing on From bindings."""

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
    name = "from",
    testonly = 1,
    srcs = ["from.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "from_golden_test",
    basename = "from",
    golden_h = "from_cc_api.h",
    golden_rs = "from_cc_api_impl.rs",
    rust_library = "from",
)

cc_bindings_from_rust(
    name = "from_cc_api",
    testonly = 1,
    crate = ":from",
)

crubit_cc_test(
    name = "from_test",
    srcs = ["from_test.cc"],
    deps = [
        ":from_cc_api",
        "@googletest//:gtest_main",
    ],
)
