"""End-to-end tests of `cc_bindings_from_rs`, focusing on bridging std::string type."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl",
    "cc_bindings_from_rust_library_config",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "string",
    srcs = ["string.rs"],
    edition = "2024",
    deps = [
        "//support/cc_std",
    ],
)

cpp_api_from_rust(
    name = "string_cc_api",
    crate = ":string",
)

cc_bindings_from_rust_library_config(
    name = "rs_namespace_string_config",
    namespace = "rs::string_bridging_test",
)

rust_library(
    name = "rs_namespace_string",
    testonly = 1,
    srcs = ["rs_namespace_string.rs"],
    aspect_hints = [":rs_namespace_string_config"],
    edition = "2024",
    deps = [
        "//support/cc_std",
    ],
)

cpp_api_from_rust(
    name = "rs_namespace_string_cc_api",
    testonly = 1,
    crate = ":rs_namespace_string",
)

crubit_cc_test(
    name = "string_test",
    srcs = ["string_test.cc"],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    # TODO(b/575815022): Re-enable once `cc_std` links on macOS.
    tags = [
        "no_macos",
        "no_windows",
    ],
    deps = [
        ":rs_namespace_string_cc_api",
        ":string_cc_api",
        "//support/rs_std:option",
        "@googletest//:gtest_main",
    ],
)
