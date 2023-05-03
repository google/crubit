"""End-to-end example using C++ libraries with dependencies."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "dependency",
    hdrs = ["dependency.h"],
)

crubit_test_cc_library(
    name = "uses_dependency",
    hdrs = ["uses_dependency.h"],
    deps = [":dependency"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":dependency",
        ":uses_dependency",
    ],
)
