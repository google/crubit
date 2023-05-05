""" Minimal example of C++ bindings for Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

# This declares an "example_crate_cc_api" target that provides Crubit-generated
# C++ bindings for the Rust crate behind the `":example_crate"` target.
rust_library(
    name = "example_crate",
    srcs = ["example.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "example_crate_cc_api",
    crate = ":example_crate",
)

cc_binary(
    name = "main",
    srcs = ["main.cc"],
    deps = [":example_crate_cc_api"],
)
