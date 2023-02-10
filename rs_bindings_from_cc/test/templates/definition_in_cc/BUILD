"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "definition_in_cc",
    srcs = ["definition_in_cc.cc"],
    hdrs = ["definition_in_cc.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":definition_in_cc"],
)
