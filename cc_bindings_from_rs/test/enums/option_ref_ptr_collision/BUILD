"""Repro: `Option<&T>` and `Option<*const T>` collide as `rs_std::Option<T const*>`."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "option_ref_ptr_collision",
    testonly = 1,
    srcs = ["option_ref_ptr_collision.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "option_ref_ptr_collision_cc_api",
    testonly = 1,
    crate = ":option_ref_ptr_collision",
)

crubit_cc_test(
    name = "option_ref_ptr_collision_test",
    srcs = ["option_ref_ptr_collision_test.cc"],
    deps = [
        ":option_ref_ptr_collision_cc_api",
        "@googletest//:gtest_main",
    ],
)
