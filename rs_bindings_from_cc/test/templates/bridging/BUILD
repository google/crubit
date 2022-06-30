"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "class_template_definition",
    hdrs = ["class_template_definition.h"],
)

cc_library(
    name = "class_template_instantiation1",
    hdrs = ["class_template_instantiation1.h"],
    deps = [":class_template_definition"],
)

cc_library(
    name = "class_template_instantiation2",
    hdrs = ["class_template_instantiation2.h"],
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
        "//rs_bindings_from_cc/support:forward_declare",
    ],
)
