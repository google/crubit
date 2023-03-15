"""End-to-end tests of C++ functions that don't have lifetime annotations."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "no_elided_lifetimes",
    srcs = ["no_elided_lifetimes.cc"],
    hdrs = ["no_elided_lifetimes.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":no_elided_lifetimes"],
)
