"""End-to-end test for operators."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "add",
    srcs = ["add.cc"],
    hdrs = ["add.h"],
)

rust_test(
    name = "add_test",
    srcs = ["add_test.rs"],
    cc_deps = [":add"],
    deps = [
        "//rs_bindings_from_cc/support:ctor",
    ],
)

cc_library(
    name = "add_assign",
    srcs = ["add_assign.cc"],
    hdrs = ["add_assign.h"],
)

rust_test(
    name = "add_assign_test",
    srcs = ["add_assign_test.rs"],
    cc_deps = [":add_assign"],
    deps = [
        "//rs_bindings_from_cc/support:ctor",
        "@crate_index//:static_assertions",
    ],
)

cc_library(
    name = "operators",
    srcs = ["operators.cc"],
    hdrs = ["operators.h"],
)

rust_test(
    name = "operators_test",
    srcs = ["operators_test.rs"],
    cc_deps = [":operators"],
    deps = [
        "//rs_bindings_from_cc/support:ctor",
        "@crate_index//:static_assertions",
    ],
)
