"""End-to-end example of using a simple inline function."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "hello_world",
    hdrs = ["hello_world.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":hello_world"],
)
