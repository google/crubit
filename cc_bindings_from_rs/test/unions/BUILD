"""End-to-end tests of `cc_bindings_from_rs`, focusing on union-related
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
    name = "unions",
    testonly = 1,
    srcs = ["unions.rs"],
    edition = "2024",
)

golden_test(
    name = "unions_golden_test",
    basename = "unions",
    golden_h = "unions_cc_api.h",
    golden_rs = "unions_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "unions",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "unions_cc_api",
    testonly = 1,
    crate = ":unions",
)

crubit_cc_test(
    name = "unions_test",
    srcs = ["unions_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":unions_cc_api",
        "@googletest//:gtest_main",
    ],
)
