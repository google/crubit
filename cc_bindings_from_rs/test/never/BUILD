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
    name = "never",
    srcs = ["never.rs"],
    edition = "2024",
)

golden_test(
    name = "never_golden_test",
    basename = "never",
    golden_h = "never_cc_api.h",
    golden_rs = "never_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "never",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "never_cc_api",
    testonly = 1,
    crate = ":never",
)

crubit_cc_test(
    name = "never_test",
    srcs = ["never_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":never_cc_api",
        "@googletest//:gtest_main",
    ],
)
