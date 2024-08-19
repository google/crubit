"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "method_params",
    hdrs = ["method_params.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":method_params"],
    deps = [
        "//third_party/gtest_rust/googletest",
    ],
)
