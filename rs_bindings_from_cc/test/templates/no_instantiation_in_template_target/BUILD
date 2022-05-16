"""End-to-end example of using type aliases that refer to fully-instantiated
templates in a different target (where the target with the header doesn't
have this particular instantiation)."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "template_without_instantiation",
    hdrs = ["template_without_instantiation.h"],
)

cc_library(
    name = "type_alias_in_different_target",
    hdrs = ["type_alias_in_different_target.h"],
    deps = [":template_without_instantiation"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":type_alias_in_different_target",
    ],
)
