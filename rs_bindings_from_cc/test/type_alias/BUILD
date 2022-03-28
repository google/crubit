"""End-to-end example of using type aliases."""

load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":type_alias"],
)
