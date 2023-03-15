"""End-to-end example of using fully-instantiated templates as function return types."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "func_return_and_param_types",
    srcs = ["func_return_and_param_types.cc"],
    hdrs = ["func_return_and_param_types.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":func_return_and_param_types"],
)
