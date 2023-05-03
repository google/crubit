"""End-to-end test for methods."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "methods",
    srcs = ["methods.cc"],
    hdrs = ["methods.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":methods"],
)
