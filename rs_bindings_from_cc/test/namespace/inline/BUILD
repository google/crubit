"""End-to-end test of inheritance."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "inline",
    hdrs = ["inline.h"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":inline"],
)
