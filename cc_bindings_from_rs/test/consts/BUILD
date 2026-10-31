"""End-to-end tests of `cc_bindings_from_rs`, focusing on consts-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "consts",
    testonly = 1,
    srcs = ["consts.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        "//support/ffi_11",
    ],
)

golden_test(
    name = "consts_golden_test",
    basename = "consts",
    golden_h = "consts_cc_api.h",
    golden_rs = "consts_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "consts",
)

cpp_api_from_rust(
    name = "consts_cc_api",
    testonly = 1,
    crate = ":consts",
)

crubit_cc_test(
    name = "consts_test",
    srcs = ["consts_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":consts_cc_api",
        "@googletest//:gtest_main",
    ],
)
