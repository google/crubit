load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
)

rust_library(
    name = "build_rs_user",
    testonly = 1,
    srcs = ["build_rs_user.rs"],
    deps = [":build_script"],
)

cc_bindings_from_rust(
    name = "build_rs_user_cc_api",
    testonly = 1,
    crate = ":build_rs_user",
)

crubit_cc_test(
    name = "build_rs_test",
    srcs = ["build_rs_test.cc"],
    deps = [
        ":build_rs_user_cc_api",
        "@googletest//:gtest_main",
    ],
)
