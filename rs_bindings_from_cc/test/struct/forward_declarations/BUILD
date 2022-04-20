"""End-to-end test of forward declarations."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "definition",
    srcs = ["definition.cc"],
    hdrs = ["definition.h"],
)

cc_library(
    name = "declaration_1",
    hdrs = ["declaration_1.h"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

cc_library(
    name = "declaration_2",
    hdrs = ["declaration_2.h"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

rust_test(
    name = "forward_declarations_test",
    srcs = ["forward_declarations_test.rs"],
    cc_deps = [
        ":declaration_1",
        ":declaration_2",
        ":definition",
    ],
    deps = [
        "//rs_bindings_from_cc/support:ctor",
        "//rs_bindings_from_cc/support:forward_declare",
    ],
)
