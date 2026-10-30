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
    name = "lifetimes",
    srcs = ["lifetimes.rs"],
    aspect_hints = [
        "//features:movable_return_values",
        "//features:supported",
    ],
    edition = "2024",
)

golden_test(
    name = "lifetimes_golden_test",
    basename = "lifetimes",
    golden_h = "lifetimes_cc_api.h",
    golden_rs = "lifetimes_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "lifetimes",
)

cpp_api_from_rust(
    name = "lifetimes_cc_api",
    testonly = 1,
    crate = ":lifetimes",
)

crubit_cc_test(
    name = "lifetimes_test",
    srcs = ["lifetimes_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":lifetimes_cc_api",
        "@googletest//:gtest_main",
    ],
)
