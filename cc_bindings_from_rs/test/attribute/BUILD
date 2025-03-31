"""End-to-end tests of `cc_bindings_from_rs`, focusing on union-related
bindings."""

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
    name = "cpp_name",
    srcs = ["cpp_name.rs"],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "cpp_name_golden_test",
    basename = "cpp_name",
    golden_h = "cpp_name_cc_api.h",
    golden_rs = "cpp_name_cc_api_impl.rs",
    rust_library = "cpp_name",
)

cc_bindings_from_rust(
    name = "cpp_name_cc_api",
    crate = ":cpp_name",
)

crubit_cc_test(
    name = "cpp_name_test",
    srcs = ["cpp_name_test.cc"],
    deps = [
        ":cpp_name_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
