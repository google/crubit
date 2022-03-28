"""End-to-end test for constructors."""

load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "constructors",
    srcs = ["constructors.cc"],
    hdrs = ["constructors.h"],
)

cc_library(
    name = "no_elided_lifetimes",
    srcs = ["no_elided_lifetimes.cc"],
    hdrs = ["no_elided_lifetimes.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":constructors",
        ":no_elided_lifetimes",
    ],
    deps = [
        "//third_party/rust/static_assertions/v1:static_assertions",
    ],
)
