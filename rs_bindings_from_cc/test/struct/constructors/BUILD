"""End-to-end test for constructors."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "constructors",
    srcs = ["constructors.cc"],
    hdrs = ["constructors.h"],
)

crubit_test_cc_library(
    name = "no_elided_lifetimes",
    srcs = ["no_elided_lifetimes.cc"],
    hdrs = ["no_elided_lifetimes.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":constructors",
        ":no_elided_lifetimes",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)
