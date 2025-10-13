"""End-to-end test for default member functions."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "default_member_functions",
    srcs = ["default_member_functions.cc"],
    hdrs = ["default_member_functions.h"],
    # operator lifetime inference and ctor
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "default_member_functions_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":default_member_functions",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
