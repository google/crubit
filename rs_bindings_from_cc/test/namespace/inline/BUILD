"""End-to-end test of inheritance."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "inline",
    hdrs = ["inline.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":inline"],
)
