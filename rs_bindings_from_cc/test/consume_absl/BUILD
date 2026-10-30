"""Consume some simple types and functions from //third_party/absl."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

rust_api_from_cpp(
    name = "time_rust",
    cpp_target = "@abseil-cpp//absl/time",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [":time_rust"],
)

crubit_test_cc_library(
    name = "absl_functional",
    hdrs = ["absl_functional.h"],
    aspect_hints = ["//features:supported"],
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
        # Required to support incomplete types.
        "//features:forward_declare",
    ],
    deps = [
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

rust_api_from_cpp(
    name = "uses_anyinvocable_rust",
    cpp_target = ":uses_anyinvocable",
)

rust_api_from_cpp(
    name = "any_invocable_rust",
    cpp_target = "@abseil-cpp//absl/functional:any_invocable",
)

crubit_rust_test(
    name = "uses_anyinvocable_test",
    srcs = ["uses_anyinvocable_test.rs"],
    deps = [
        ":any_invocable_rust",
        ":uses_anyinvocable_rust",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "any_invocable_alias",
    hdrs = ["any_invocable_alias.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

# Depends on the AnyInvocable bridge only through `:any_invocable_alias`. Checks
# that the generated bindings can include the bridge's support header.
crubit_test_cc_library(
    name = "uses_any_invocable_alias",
    hdrs = ["uses_any_invocable_alias.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        ":any_invocable_alias",
    ],
)

rust_api_from_cpp(
    name = "uses_any_invocable_alias_rust",
    cpp_target = ":uses_any_invocable_alias",
)

crubit_rust_test(
    name = "uses_any_invocable_alias_test",
    srcs = ["uses_any_invocable_alias_test.rs"],
    deps = [
        ":uses_any_invocable_alias_rust",
        "@crate_index//:googletest",
    ],
)

rust_api_from_cpp(
    name = "int128_rust",
    cpp_target = "@abseil-cpp//absl/numeric:int128",
)

crubit_rust_test(
    name = "int128_test",
    srcs = ["int128_test.rs"],
    deps = [
        ":int128_rust",
        "@crate_index//:googletest",
    ],
)
