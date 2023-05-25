"""Consume some simple types and functions from //third_party/absl."""

load("//rs_bindings_from_cc/test:crubit_rust_test.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = ["@absl//absl/time"],
)
