"""End-to-end test for destructors."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "field_destruction_order",
    srcs = ["field_destruction_order.cc"],
    hdrs = ["field_destruction_order.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":field_destruction_order"],
)
