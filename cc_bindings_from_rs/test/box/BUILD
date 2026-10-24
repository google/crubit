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

rust_library(
    name = "boxes",
    srcs = ["boxes.rs"],
    aspect_hints = ["//features:movable_return_values"],
    edition = "2024",
)

golden_test(
    name = "boxes_golden_test",
    basename = "boxes",
    golden_h = "boxes_cc_api.h",
    golden_rs = "boxes_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "boxes",
)

cpp_api_from_rust(
    name = "boxes_cc_api",
    testonly = 1,
    crate = ":boxes",
)

crubit_cc_test(
    name = "boxes_test",
    srcs = ["boxes_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":boxes_cc_api",
        "//support:movable",
        "@googletest//:gtest_main",
    ],
)
