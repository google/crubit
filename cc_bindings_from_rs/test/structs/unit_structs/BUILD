"""End-to-end tests of `cc_bindings_from_rs`, focusing on unit-struct-related
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
    name = "unit_structs",
    testonly = 1,
    srcs = ["unit_structs.rs"],
    edition = "2024",
)

golden_test(
    name = "unit_structs_golden_test",
    basename = "unit_structs",
    golden_h = "unit_structs_cc_api.h",
    golden_rs = "unit_structs_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "unit_structs",
)

cpp_api_from_rust(
    name = "unit_structs_cc_api",
    testonly = 1,
    crate = ":unit_structs",
)

crubit_cc_test(
    name = "unit_structs_test",
    srcs = ["unit_structs_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":unit_structs_cc_api",
        "@googletest//:gtest_main",
    ],
)
