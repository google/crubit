"""End-to-end example of using type aliases."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "void_pointers",
    hdrs = ["void_pointers.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":void_pointers"],
)
