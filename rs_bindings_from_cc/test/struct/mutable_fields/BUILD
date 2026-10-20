"""End-to-end test of using a simple struct."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "mutable_fields",
    hdrs = ["mutable_fields.h"],
)

rust_api_from_cpp(
    name = "mutable_fields_rust",
    cpp_target = ":mutable_fields",
    tags = [
        "manual",
        "notap",
    ],
)

crubit_rust_test(
    name = "mutable_fields_test",
    srcs = ["test.rs"],
    # TODO(b/481398972): Re-enable this test as proof the bug is fixed.
    tags = [
        "manual",
        "notap",
    ],
    deps = [
        ":mutable_fields_rust",
        "@crate_index//:googletest",
    ],
)
