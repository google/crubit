"""End-to-end example of using type aliases."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "void_pointers",
    hdrs = ["void_pointers.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":void_pointers"],
)
