"""End-to-end tests of `cc_bindings_from_rs`, focusing on type aliases bindings."""

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
    name = "type_aliases",
    testonly = 1,
    srcs = ["type_aliases.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    edition = "2024",
    deps = [
        "//support:bridge_rust",
    ],
)

cpp_api_from_rust(
    name = "type_aliases_cc_api",
    testonly = 1,
    crate = ":type_aliases",
)

golden_test(
    name = "type_aliases_golden_test",
    basename = "type_aliases",
    golden_h = "type_aliases_cc_api.h",
    golden_rs = "type_aliases_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "type_aliases",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "type_aliases_test",
    srcs = ["type_aliases_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":type_aliases_cc_api",
        "@googletest//:gtest_main",
    ],
)
