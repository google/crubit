"""End-to-end example using C++ libraries with dependencies."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "dependency",
    hdrs = ["dependency.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "uses_dependency",
    hdrs = ["uses_dependency.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":dependency"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":dependency",
        ":uses_dependency",
    ],
)
