"""End-to-end tests of `cc_bindings_from_rs`, focusing on union-related
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
    name = "cpp_name",
    srcs = ["cpp_name.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "cpp_name_golden_test",
    basename = "cpp_name",
    golden_h = "cpp_name_cc_api.h",
    golden_rs = "cpp_name_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "cpp_name",
)

cpp_api_from_rust(
    name = "cpp_name_cc_api",
    crate = ":cpp_name",
)

crubit_cc_test(
    name = "cpp_name_test",
    srcs = ["cpp_name_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":cpp_name_cc_api",
        "@googletest//:gtest_main",
    ],
)

rust_library(
    name = "must_bind",
    srcs = ["must_bind.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "must_bind_golden_test",
    basename = "must_bind",
    golden_h = "must_bind_cc_api.h",
    golden_rs = "must_bind_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "must_bind",
)

cpp_api_from_rust(
    name = "must_bind_cc_api",
    crate = ":must_bind",
)

crubit_cc_test(
    name = "must_bind_test",
    srcs = ["must_bind_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":must_bind_cc_api",
        "@googletest//:gtest_main",
    ],
)
