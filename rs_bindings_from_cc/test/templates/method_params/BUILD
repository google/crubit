"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "method_params",
    hdrs = ["method_params.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":method_params"],
)
