"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "out_of_line_definition",
    hdrs = ["out_of_line_definition.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":out_of_line_definition"],
)
