"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "extern_definition",
    hdrs = ["extern_definition.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "actual_instantiation",
    hdrs = ["actual_instantiation.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":extern_definition"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":actual_instantiation",
        ":extern_definition",
    ],
)
