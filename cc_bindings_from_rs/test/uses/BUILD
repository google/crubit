"""End-to-end tests of `cc_bindings_from_rs`, focusing on use statements."""

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
    name = "extern_crate",
    testonly = 1,
    srcs = ["extern_crate.rs"],
    edition = "2024",
)

rust_library(
    name = "uses",
    testonly = 1,
    srcs = ["uses.rs"],
    edition = "2024",
    proc_macro_deps = ["//support:crubit_annotate"],
    deps = [":extern_crate"],
)

cpp_api_from_rust(
    name = "uses_cc_api",
    testonly = 1,
    crate = ":uses",
)

crubit_cc_test(
    name = "uses_test",
    srcs = ["uses_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":uses_cc_api",
        "@googletest//:gtest_main",
    ],
)

golden_test(
    name = "uses_golden_test",
    basename = "uses",
    golden_h = "uses_cc_api.h",
    golden_rs = "uses_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "uses",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)
