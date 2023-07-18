"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//rs_bindings_from_cc/test:crubit_rust_test.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "non_type_template_params",
    hdrs = ["non_type_template_params.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":non_type_template_params"],
)
