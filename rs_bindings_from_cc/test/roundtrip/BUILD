"""End-to-end example of using simple functions."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "roundtrip",
    hdrs = ["roundtrip.h"],
    deps = [
        "//support/rs_std:rs_char",
    ],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":roundtrip"],
)
