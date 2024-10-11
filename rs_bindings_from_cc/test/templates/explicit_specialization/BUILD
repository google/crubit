"""End-to-end example of explicit template specialization working."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "explicit_specialization",
    hdrs = ["explicit_specialization.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":explicit_specialization",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
