"""End-to-end example using C++ libraries with dependencies."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "dependency",
    hdrs = ["dependency.h"],
)

cc_library(
    name = "uses_dependency",
    hdrs = ["uses_dependency.h"],
    deps = [":dependency"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":dependency",
        ":uses_dependency",
    ],
)
