"""End-to-end test of ABI classification of structs."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "abi_class",
    hdrs = ["abi_class.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":abi_class"],
    deps = [
        "@crate_index//:googletest",
    ],
)
