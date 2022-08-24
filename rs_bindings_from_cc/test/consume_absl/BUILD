"""Consume some simple types and functions from //third_party/absl."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = ["@absl//absl/time"],
    # TODO(214901011): Reenable when bindings generation doesn't crash
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)
