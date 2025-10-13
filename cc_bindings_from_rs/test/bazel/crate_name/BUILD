"""End-to-end tests of `cc_bindings_from_rs`, focusing on how
Bazel<->cc_bindings_from_rs integration handles a custom `name` in a
`rust_library` target."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

rust_library(
    name = "custom_crate_name",  # <- This is an important part of this test.
    testonly = 1,
    srcs = ["lib.rs"],
)

cc_bindings_from_rust(
    name = "custom_crate_name_cc_api",
    testonly = 1,
    crate = ":custom_crate_name",
)

crubit_cc_test(
    name = "crate_name_test",
    srcs = ["crate_name_test.cc"],
    deps = [
        ":custom_crate_name_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
