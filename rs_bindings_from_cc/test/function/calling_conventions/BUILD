"""End-to-end tests of C++ functions with non-default calling conventions."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "calling_conventions",
    srcs = ["calling_conventions.cc"],
    hdrs = ["calling_conventions.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":calling_conventions"],
)
