"""End-to-end test for destructors."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "field_destruction_order",
    srcs = ["field_destruction_order.cc"],
    hdrs = ["field_destruction_order.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":field_destruction_order"],
    deps = [
        "//third_party/gtest_rust/googletest",
    ],
)
