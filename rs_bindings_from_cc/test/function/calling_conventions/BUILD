"""End-to-end tests of C++ functions with non-default calling conventions."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "calling_conventions",
    srcs = ["calling_conventions.cc"],
    hdrs = ["calling_conventions.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":calling_conventions"],
    deps = [
        "@crates//:googletest",
    ],
)
