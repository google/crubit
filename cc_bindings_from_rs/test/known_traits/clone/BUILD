"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Clone` trait"""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//cc_bindings_from_rs:test/crubit_cc_test.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    # `name = "clone"` would make the bindings conflict with a top-level item in the C++ standard library
    # See also b/287222576.
    name = "rs_clone",
    testonly = 1,
    srcs = ["clone.rs"],
)

cc_bindings_from_rust(
    name = "rs_clone_cc_api",
    testonly = 1,
    crate = ":rs_clone",
)

crubit_cc_test(
    name = "clone_test",
    srcs = ["clone_test.cc"],
    deps = [
        ":rs_clone_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
