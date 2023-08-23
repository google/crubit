"""Consume some simple types and functions from //third_party/absl."""

load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = ["@absl//absl/time"],
)
