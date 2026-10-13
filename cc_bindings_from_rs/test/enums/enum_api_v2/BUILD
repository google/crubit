"""End-to-end tests of `cc_bindings_from_rs` with the `enum_api_v2` feature."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "enum_api_v2",
    testonly = 1,
    srcs = ["enum_api_v2.rs"],
    aspect_hints = [
        "//features:enum_api_v2",
        "//features:experimental",
    ],
    edition = "2024",
)

cc_bindings_from_rust(
    name = "enum_api_v2_cc_api",
    testonly = 1,
    crate = ":enum_api_v2",
)

crubit_cc_test(
    name = "enum_api_v2_test",
    srcs = ["enum_api_v2_test.cc"],
    deps = [
        ":enum_api_v2_cc_api",
        "//support/rs_std:enum",
        "//testing/base/public:gunit_main",
    ],
)
