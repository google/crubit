"""End-to-end tests of `cc_bindings_from_rs`, focusing on use statements."""

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
    name = "transitive",
    testonly = 1,
    srcs = ["transitive.rs"],
    edition = "2024",
)

rust_library(
    name = "direct",
    testonly = 1,
    srcs = ["direct.rs"],
    edition = "2024",
    deps = [":transitive"],
)

rust_library(
    name = "transitive_reexports",
    testonly = 1,
    srcs = ["transitive_reexports.rs"],
    edition = "2024",
    deps = [":direct"],
)

cpp_api_from_rust(
    name = "transitive_reexports_cc_api",
    testonly = 1,
    crate = ":transitive_reexports",
)

crubit_cc_test(
    name = "transitive_reexports_test",
    srcs = ["transitive_reexports_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":transitive_reexports_cc_api",
        "@googletest//:gtest_main",
    ],
)

golden_test(
    name = "transitive_reexports_golden_test",
    basename = "transitive_reexports",
    golden_h = "transitive_reexports_cc_api.h",
    golden_rs = "transitive_reexports_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "transitive_reexports",
)
