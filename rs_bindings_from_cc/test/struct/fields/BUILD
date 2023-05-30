"""End-to-end test of using a simple struct."""


load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "fields",
    hdrs = ["fields.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":fields"],
)
