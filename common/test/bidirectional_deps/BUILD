load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

rust_library(
    name = "leaf_rs_lib",
    srcs = ["leaf_rs_lib.rs"],
)

cc_bindings_from_rust(
    name = "leaf_rs_lib_cc_api",
    crate = ":leaf_rs_lib",
)

crubit_test_cc_library(
    name = "middle_cc_lib",
    hdrs = ["middle_cc_lib.h"],
    deps = [
        ":leaf_rs_lib_cc_api",
    ],
)

crubit_rust_test(
    name = "rs_test",
    srcs = ["rs_test.rs"],
    cc_deps = [
        ":middle_cc_lib",
    ],
    deps = [
        ":leaf_rs_lib",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "leaf_cc_lib",
    hdrs = ["leaf_cc_lib.h"],
)

rust_library(
    name = "middle_rs_lib",
    srcs = ["middle_rs_lib.rs"],
    cc_deps = [":leaf_cc_lib"],
)

cc_bindings_from_rust(
    name = "middle_rs_lib_cc_api",
    crate = ":middle_rs_lib",
)

crubit_cc_test(
    name = "cc_test",
    srcs = ["cc_test.cc"],
    deps = [
        ":leaf_cc_lib",
        ":middle_rs_lib_cc_api",
        "@googletest//:gtest_main",
    ],
)
