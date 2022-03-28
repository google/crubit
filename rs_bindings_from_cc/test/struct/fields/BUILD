"""End-to-end test of using a simple struct."""

load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "fields",
    hdrs = ["fields.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":fields"],
)
