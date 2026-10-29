"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Index` trait"""

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
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "rs_index",
    testonly = 1,
    srcs = ["rs_index.rs"],
    aspect_hints = ["//features:movable_return_values"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "rs_index_cc_api",
    testonly = 1,
    crate = ":rs_index",
)

golden_test(
    name = "rs_index_cc_api_golden",
    basename = "rs_index",
    golden_h = "rs_index_cc_api.h",
    golden_rs = "rs_index_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "rs_index",
)

crubit_cc_test(
    name = "rs_index_test",
    srcs = ["rs_index_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":rs_index_cc_api",
        "@googletest//:gtest_main",
    ],
)
