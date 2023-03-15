"""End-to-end example of using type aliases that refer to fully-instantiated
templates in a different target (where the target with the header doesn't
have this particular instantiation)."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "template_without_instantiation",
    hdrs = ["template_without_instantiation.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "type_alias_in_different_target",
    hdrs = ["type_alias_in_different_target.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":template_without_instantiation"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":type_alias_in_different_target",
    ],
)
