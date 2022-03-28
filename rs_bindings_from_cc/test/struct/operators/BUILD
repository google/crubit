"""End-to-end test for operators."""

load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "operators",
    srcs = ["operators.cc"],
    hdrs = ["operators.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":operators"],
    deps = [
        "//third_party/rust/static_assertions/v1:static_assertions",
    ],
)
