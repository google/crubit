"""End-to-end test of inheritance."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "inline",
    hdrs = ["inline.h"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":inline"],
)
