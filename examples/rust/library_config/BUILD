load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
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

# This configuration places all names inside the namespace `my::library`, instead of the crate name.
cc_bindings_from_rust_library_config(
    name = "custom_namespace",
    namespace = "my::library",
)

# This declares an "example_crate_cc_api" target that provides Crubit-generated
# C++ bindings for the Rust crate behind the `":example_crate"` target.
rust_library(
    name = "example_crate",
    srcs = ["example.rs"],
    aspect_hints = [":custom_namespace"],
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

golden_test(
    name = "example_golden_test",
    golden_h = "example_generated.h",
    rust_library = "example_crate",
)
