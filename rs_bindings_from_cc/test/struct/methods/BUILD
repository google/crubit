"""End-to-end test for methods."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "methods",
    srcs = ["methods.cc"],
    hdrs = ["methods.h"],
    aspect_hints = ["//features:assume_this_lifetimes"],
)

golden_test(
    name = "methods_golden_test",
    basename = "methods",
    cc_library = ":methods",
    golden_cc = "methods_api_impl.cc",
    golden_rs = "methods_rs_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":methods"],
    deps = [
        "@crate_index//:googletest",
    ],
)
