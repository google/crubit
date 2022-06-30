"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
)

cc_library(
    name = "type_alias_in_different_target",
    hdrs = ["type_alias_in_different_target.h"],
    deps = [":type_alias"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":type_alias",
        ":type_alias_in_different_target",
    ],
    deps = [
        "//rs_bindings_from_cc/support:forward_declare",
    ],
)
