"""Repro for b/555410712: ABSL_REQUIRE_EXPLICIT_INIT and Crubit bindings."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "require_explicit_init",
    hdrs = ["require_explicit_init.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["@abseil-cpp//absl/base:core_headers"],
)

crubit_rust_test(
    name = "require_explicit_init_test",
    srcs = ["test.rs"],
    cc_deps = [":require_explicit_init"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
