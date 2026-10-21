"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Default` trait"""

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
    # `default` is a reserved C++ keyword, so using `rs_` prefix.
    name = "rs_default",
    testonly = 1,
    srcs = ["default.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "rs_default_cc_api",
    testonly = 1,
    crate = ":rs_default",
)

golden_test(
    name = "default_golden_test",
    basename = "default",
    golden_h = "default_cc_api.h",
    golden_rs = "default_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "rs_default",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "default_test",
    srcs = ["default_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":rs_default_cc_api",
        "@googletest//:gtest_main",
    ],
)
