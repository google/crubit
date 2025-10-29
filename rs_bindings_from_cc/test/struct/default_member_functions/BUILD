"""End-to-end test for default member functions."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "default_member_functions",
    srcs = ["default_member_functions.cc"],
    hdrs = ["default_member_functions.h"],
    aspect_hints = ["//features:infer_operator_lifetimes"],
)

golden_test(
    name = "default_member_functions_golden_test",
    basename = "default_member_functions",
    cc_library = "default_member_functions",
    golden_cc = "default_member_functions_api_impl.cc",
    golden_rs = "default_member_functions_rs_api.rs",
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
