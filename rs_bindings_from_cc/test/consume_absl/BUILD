"""Consume some simple types and functions from //third_party/absl."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = ["@absl//absl/time"],
)
