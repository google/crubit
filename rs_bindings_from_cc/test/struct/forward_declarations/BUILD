"""End-to-end test of forward declarations."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "definition",
    hdrs = ["definition.h"],
)

rust_test(
    name = "forward_declarations_test",
    srcs = ["forward_declarations_test.rs"],
    cc_deps = [":definition"],
    deps = [
        "//rs_bindings_from_cc/support:ctor",
        "//rs_bindings_from_cc/support:forward_declare",
    ],
)
