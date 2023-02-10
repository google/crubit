"""End-to-end example of using fully-instantiated templates as function return types."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "struct_fields",
    hdrs = ["struct_fields.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":struct_fields"],
    deps = ["//support:ctor"],
)
