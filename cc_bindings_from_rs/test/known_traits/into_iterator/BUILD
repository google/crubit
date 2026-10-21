# End-to-end tests of `cc_bindings_from_rs`, focusing on the `IntoIterator` trait

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
    name = "into_iterator_rust",
    testonly = 1,
    srcs = ["into_iterator.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    edition = "2024",
    deps = [
        "//support:bridge_rust",
    ],
)

golden_test(
    name = "into_iterator_golden_test",
    basename = "into_iterator",
    golden_h = "into_iterator_cc_api.h",
    golden_rs = "into_iterator_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "into_iterator_rust",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "into_iterator_cc_api",
    testonly = 1,
    crate = ":into_iterator_rust",
)

crubit_cc_test(
    name = "into_iterator_cc_test",
    srcs = ["into_iterator_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":into_iterator_cc_api",
        "@googletest//:gtest_main",
    ],
)
