"""End-to-end test for destructors."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "field_destruction_order",
    srcs = ["field_destruction_order.cc"],
    hdrs = ["field_destruction_order.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":field_destruction_order"],
)
