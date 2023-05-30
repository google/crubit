"""End-to-end example of using type aliases to fully-instantiated templates."""


load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "two_template_parameters",
    hdrs = ["two_template_parameters.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":two_template_parameters"],
)
