"""End-to-end test for constructors."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

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
        "//support:ctor",
        "@crate_index//:static_assertions",
    ],
)
