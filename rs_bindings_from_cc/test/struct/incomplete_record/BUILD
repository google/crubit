"""End-to-end test of forward declarations."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "reference_incomplete_record_by_value",
    hdrs = [
        "reference_incomplete_record_by_value.h",
    ],
)

rust_api_from_cpp(
    name = "reference_incomplete_record_by_value_rust",
    cpp_target = ":reference_incomplete_record_by_value",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":reference_incomplete_record_by_value_rust",
        "@crate_index//:googletest",
    ],
)
