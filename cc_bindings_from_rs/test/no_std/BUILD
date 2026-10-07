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
load(
    "//support/rs_std:crubit_stdlib_cc_test.bzl",
    "crubit_stdlib_cc_test",
)

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "no_std",
    srcs = ["no_std.rs"],
    edition = "2024",
)

golden_test(
    name = "no_std_golden_test",
    basename = "no_std",
    golden_h = "no_std_cc_api.h",
    golden_rs = "no_std_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "no_std",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "no_std_cc_api",
    testonly = 1,
    crate = ":no_std",
)

crubit_cc_test(
    name = "no_std_test",
    srcs = ["no_std_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":no_std_cc_api",
        "@googletest//:gtest_main",
    ],
)

crubit_stdlib_cc_test(
    name = "no_std_test_on_alloc",
    srcs = ["no_std_test.cc"],
    stdlib = "alloc",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":no_std_cc_api",
        "@googletest//:gtest_main",
    ],
)

rust_library(
    name = "no_core",
    srcs = ["no_core.rs"],
    edition = "2024",
)

golden_test(
    name = "no_core_golden_test",
    basename = "no_core",
    golden_h = "no_core_cc_api.h",
    golden_rs = "no_core_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "no_core",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "no_core_cc_api",
    testonly = 1,
    crate = ":no_core",
)

crubit_cc_test(
    name = "no_core_test",
    srcs = ["no_core_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":no_core_cc_api",
        "@googletest//:gtest_main",
    ],
)

crubit_stdlib_cc_test(
    name = "no_core_test_on_core",
    srcs = ["no_core_test.cc"],
    stdlib = "core",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":no_core_cc_api",
        "@googletest//:gtest_main",
    ],
)
