"""Consume some simple types and functions from //third_party/absl."""

load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_test")

licenses(["notice"])

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = ["//third_party/absl/time"],
)
