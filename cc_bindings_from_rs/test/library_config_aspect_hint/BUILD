"""End-to-end tests of `cc_bindings_from_rs`, focusing on the namespace configuration of the `cc_bindings_from_rust_library_config` aspect hint."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_bindings_from_rust_library_config(
    name = "namespace_crate1_config",
    namespace = "crate1_namespace::crate1_subnamespace",
)

rust_library(
    name = "namespace_crate1",
    testonly = 1,
    srcs = ["namespace_crate1.rs"],
    aspect_hints = [":namespace_crate1_config"],
)

cc_bindings_from_rust_library_config(
    name = "namespace_crate2_config",
    namespace = "crate2_namespace::crate2_subnamespace",
)

rust_library(
    name = "namespace_crate3",
    testonly = 1,
    srcs = ["namespace_crate3.rs"],
)

rust_library(
    name = "namespace_crate2",
    testonly = 1,
    srcs = ["namespace_crate2.rs"],
    aspect_hints = [":namespace_crate2_config"],
    deps = [
        ":namespace_crate1",
        ":namespace_crate3",
    ],
)

cc_bindings_from_rust(
    name = "namespace_crate2_cc_api",
    testonly = 1,
    crate = ":namespace_crate2",
)

crubit_cc_test(
    name = "namespace_test",
    srcs = ["namespace_test.cc"],
    deps = [
        ":namespace_crate2_cc_api",
        "@googletest//:gtest_main",
    ],
)
