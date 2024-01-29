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
    name = "primitive_types",
    testonly = 1,
    srcs = ["primitive_types.rs"],
    deps = ["@crate_index//:libc"],
)

cc_bindings_from_rust(
    name = "primitive_types_cc_api",
    testonly = 1,
    crate = ":primitive_types",
)

crubit_cc_test(
    name = "primitive_types_test",
    srcs = ["primitive_types_test.cc"],
    deps = [
        ":primitive_types_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
