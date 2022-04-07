"""End-to-end test for methods."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "methods",
    srcs = ["methods.cc"],
    hdrs = ["methods.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":methods"],
)
