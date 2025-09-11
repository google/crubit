"""End-to-end test for constructors."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "constructors",
    srcs = ["constructors.cc"],
    hdrs = ["constructors.h"],
    # operator lifetime inference and ctor
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "constructors_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":constructors",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)
