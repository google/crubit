"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "class_template_definition",
    hdrs = ["class_template_definition.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "class_template_instantiation1",
    hdrs = ["class_template_instantiation1.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":class_template_definition"],
)

cc_library(
    name = "class_template_instantiation2",
    hdrs = ["class_template_instantiation2.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":class_template_definition"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":class_template_instantiation1",
        ":class_template_instantiation2",
    ],
    deps = [
        "//support:forward_declare",
    ],
)
