load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "function_pointers",
    srcs = ["function_pointers.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    proc_macro_deps = ["//support:crubit_annotate"],
)

golden_test(
    name = "function_pointers_golden_test",
    basename = "function_pointers",
    golden_h = "function_pointers_cc_api.h",
    golden_rs = "function_pointers_cc_api_impl.rs",
    rust_library = "function_pointers",
)

cc_bindings_from_rust(
    name = "function_pointers_cc_api",
    testonly = 1,
    crate = ":function_pointers",
)

crubit_cc_test(
    name = "function_pointers_test",
    srcs = ["function_pointers_test.cc"],
    deps = [
        ":function_pointers_cc_api",
        "@googletest//:gtest_main",
    ],
)
