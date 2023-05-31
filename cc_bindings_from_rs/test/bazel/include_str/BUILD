"""End-to-end tests of `cc_bindings_from_rs`, focusing on how
Bazel<->cc_bindings_from_rs integration handles `include_str`
(i.e. a `data` / non-`src` dependency)."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)


rust_library(
    name = "include_str",
    testonly = 1,
    srcs = ["include_str.rs"],
    data = ["the_answer.md"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

cc_bindings_from_rust(
    name = "include_str_cc_api",
    testonly = 1,
    crate = ":include_str",
)

cc_test(
    name = "include_str_test",
    srcs = ["include_str_test.cc"],
    deps = [
        ":include_str_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
