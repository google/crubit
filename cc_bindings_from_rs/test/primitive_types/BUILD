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
    name = "primitive_types",
    testonly = 1,
    srcs = ["primitive_types.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    edition = "2024",
    deps = [
        "//support/ffi_11",
        "@crate_index//:libc",  # v0_2
    ],
)

cpp_api_from_rust(
    name = "primitive_types_cc_api",
    testonly = 1,
    crate = ":primitive_types",
)

golden_test(
    name = "primitive_types_golden_test",
    basename = "primitive_types",
    golden_h = "primitive_types_cc_api.h",
    golden_rs = "primitive_types_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "primitive_types",
)

crubit_cc_test(
    name = "primitive_types_test",
    srcs = ["primitive_types_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":primitive_types_cc_api",
        "@googletest//:gtest_main",
    ],
)
