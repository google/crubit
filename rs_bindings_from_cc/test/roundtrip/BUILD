"""End-to-end example of using simple functions."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
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
