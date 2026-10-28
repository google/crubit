"""End-to-end tests of `cc_bindings_from_rs`, focusing on Into bindings."""

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
    name = "into",
    testonly = 1,
    srcs = ["into.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "into_golden_test",
    basename = "into",
    golden_h = "into_cc_api.h",
    golden_rs = "into_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "into",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "into_cc_api",
    testonly = 1,
    crate = ":into",
)

crubit_cc_test(
    name = "into_test",
    srcs = ["into_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":into_cc_api",
        "@googletest//:gtest_main",
    ],
)
