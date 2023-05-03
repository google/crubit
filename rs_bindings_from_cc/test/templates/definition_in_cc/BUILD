"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "definition_in_cc",
    srcs = ["definition_in_cc.cc"],
    hdrs = ["definition_in_cc.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":definition_in_cc"],
)
