"""End-to-end example of using simple functions."""

load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "simple_functions",
    srcs = ["simple_functions.cc"],
    hdrs = ["simple_functions.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":simple_functions"],
)
