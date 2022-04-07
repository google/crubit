"""End-to-end tests of C++ functions with non-default calling conventions."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "calling_conventions",
    srcs = ["calling_conventions.cc"],
    hdrs = ["calling_conventions.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":calling_conventions"],
)
