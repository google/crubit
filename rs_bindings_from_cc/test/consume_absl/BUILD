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
    aspect_hints = ["//features:callables"],
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

crubit_test_cc_library(
    name = "uses_anyinvocable",
    srcs = ["uses_anyinvocable.cc"],
    hdrs = ["uses_anyinvocable.h"],
    aspect_hints = [
        "//features:supported",
        "//features:callables",
    ],
    deps = [
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

crubit_rust_test(
    name = "uses_anyinvocable_test",
    srcs = ["uses_anyinvocable_test.rs"],
    cc_deps = [
        ":uses_anyinvocable",
        "@abseil-cpp//absl/functional:any_invocable",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
