"""End-to-end example of using simple functions."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "simple_functions",
    srcs = ["simple_functions.cc"],
    hdrs = ["simple_functions.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":simple_functions"],
)
