"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
)

crubit_test_cc_library(
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
        "//support:forward_declare",
    ],
)
