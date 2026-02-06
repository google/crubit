"""Consume some simple types and functions from //third_party/absl."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = ["@abseil-cpp//absl/time"],
)

crubit_test_cc_library(
    name = "absl_functional",
    hdrs = ["absl_functional.h"],
    deps = [
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

golden_test(
    name = "absl_functional_golden_test",
    cc_library = "absl_functional",
    golden_cc = "absl_functional.golden.cc",
    golden_rs = "absl_functional.golden.rs",
)
