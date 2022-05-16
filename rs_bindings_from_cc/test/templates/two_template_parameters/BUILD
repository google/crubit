"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "two_template_parameters",
    hdrs = ["two_template_parameters.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":two_template_parameters"],
)
