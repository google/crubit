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
    deps = [":example_crate_cc_api"],
)

golden_test(
    name = "example_golden_test",
    golden_h = "example_generated.h",
    rust_library = "example_crate",
)
