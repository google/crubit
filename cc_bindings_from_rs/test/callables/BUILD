"""End-to-end tests of `cc_bindings_from_rs`, focusing on callable parameter bindings."""

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
    name = "callables",
    srcs = ["callables.rs"],
    aspect_hints = [
        "//features:movable_return_values",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "callables_golden_test",
    basename = "callables",
    golden_h = "callables_cc_api.h",
    golden_rs = "callables_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "callables",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "callables_cc_api",
    testonly = 1,
    crate = ":callables",
)

crubit_cc_test(
    name = "callables_test",
    srcs = ["callables_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":callables_cc_api",
        "@googletest//:gtest_main",
    ],
)
