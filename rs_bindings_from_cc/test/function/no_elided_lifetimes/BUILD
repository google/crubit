"""End-to-end tests of C++ functions that don't have lifetime annotations."""

load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "no_elided_lifetimes",
    srcs = ["no_elided_lifetimes.cc"],
    hdrs = ["no_elided_lifetimes.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":no_elided_lifetimes"],
)
