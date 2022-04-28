"""End-to-end test of inheritance."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "upcast",
    hdrs = ["upcast.h"],
)

rust_test(
    name = "upcast_test",
    srcs = ["upcast_test.rs"],
    cc_deps = [":upcast"],
    deps = [
        "//rs_bindings_from_cc/support:ctor",
    ],
)
