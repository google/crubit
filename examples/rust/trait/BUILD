load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

# This declares an "example_crate_cc_api" target that provides Crubit-generated
# C++ bindings for the Rust crate behind the `":example_crate"` target.
rust_library(
    name = "example_crate",
    srcs = ["example.rs"],
)

cc_bindings_from_rust(
    name = "example_crate_cc_api",
    crate = ":example_crate",
)

cc_binary(
    name = "main",
    srcs = ["main.cc"],
    deps = [
        ":example_crate_cc_api",
        "//support/rs_std:str_ref",
    ],
)

golden_test(
    name = "example_golden_test",
    golden_h = "example_generated.h",
    rust_library = "example_crate",
)
