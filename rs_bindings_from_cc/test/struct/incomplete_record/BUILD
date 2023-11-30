"""End-to-end test of forward declarations."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "reference_incomplete_record_by_value",
    hdrs = [
        "reference_incomplete_record_by_value.h",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":reference_incomplete_record_by_value",
    ],
    tags = [
        # b/314152421: It currently crashes Crubit with:
        # IncompleteRecord is unexpected as a parameter type or return type.
        "manual",
        "nobuilder",
        "notap",
    ],
)
