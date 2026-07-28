"""End-to-end tests of `cc_bindings_from_rs`, focusing on cross-crate bindings
(bindings for items that refer to types from another crate)."""

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
    name = "other_crate_hint",
    extra_cc_hdrs = ["other_crate_types.h"],
)

rust_library(
    name = "other_crate",
    testonly = 1,
    srcs = ["other_crate.rs"],
    aspect_hints = [
        ":other_crate_hint",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust(
    name = "other_crate_cc_api",
    testonly = 1,
    crate = ":other_crate",
)

rust_library(
    name = "test_api",
    testonly = 1,
    srcs = ["test_api.rs"],
    deps = [":other_crate"],
)

cc_bindings_from_rust(
    name = "test_api_cc_api",
    testonly = 1,
    crate = ":test_api",
)

crubit_cc_test(
    name = "cross_crate_test",
    srcs = ["cross_crate_test.cc"],
    platforms = ["android"],
    deps = [
        ":other_crate_cc_api",
        ":test_api_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
