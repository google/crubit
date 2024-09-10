"""End-to-end tests for header renaming."""

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
    name = "rename_lib",
    testonly = 1,
    srcs = ["rename_lib.rs"],
)

cc_bindings_from_rust(
    name = "rename_lib_cc_api",
    testonly = 1,
    crate = ":rename_lib",
)

crubit_cc_test(
    name = "old_name_test",
    srcs = ["old_name_test.cc"],
    deps = [
        ":rename_lib_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "new_name_test",
    srcs = ["new_name_test.cc"],
    deps = [
        ":rename_lib_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "both_names_test",
    srcs = ["both_names_test.cc"],
    deps = [
        ":rename_lib_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
