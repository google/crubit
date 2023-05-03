"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "extern_definition",
    hdrs = ["extern_definition.h"],
)

crubit_test_cc_library(
    name = "actual_instantiation",
    hdrs = ["actual_instantiation.h"],
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
