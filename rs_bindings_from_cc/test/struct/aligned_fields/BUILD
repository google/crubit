"""End-to-end test of structs with explicitly aligned fields."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "aligned_fields",
    hdrs = ["aligned_fields.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":aligned_fields"],
    deps = [
        "@crate_index//:googletest",
    ],
)
