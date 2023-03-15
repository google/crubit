"""End-to-end test of forward declarations."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "definition",
    srcs = ["definition.cc"],
    hdrs = ["definition.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "declaration_1",
    hdrs = ["declaration_1.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

cc_library(
    name = "declaration_2",
    hdrs = ["declaration_2.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

cc_library(
    name = "no_definition_in_headers",
    hdrs = ["no_definition_in_headers.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "forward_declarations_test",
    srcs = ["forward_declarations_test.rs"],
    cc_deps = [
        ":declaration_1",
        ":declaration_2",
        ":definition",
        ":no_definition_in_headers",
    ],
    deps = [
        "//support:ctor",
        "//support:forward_declare",
    ],
)
