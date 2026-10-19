"""End-to-end tests of `cc_bindings_from_rs`, focusing on the namespace configuration of the `cc_bindings_from_rust_library_config` aspect hint."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)

# We use cpp_api_from_rust_library_config for crate1 since it only receives bindings transitively.
load("//bazel:cpp_api_from_rust_library_config.bzl", "cpp_api_from_rust_library_config")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")
load(":config_hint_test.bzl", "duplicate_hint_test", "missing_hint_test")

package(default_applicable_licenses = ["//:license"])

cpp_api_from_rust_library_config(
    name = "namespace_crate1_config",
    namespace = "crate1_namespace::crate1_subnamespace",
)

rust_library(
    name = "namespace_crate1",
    testonly = 1,
    srcs = ["namespace_crate1.rs"],
    aspect_hints = [":namespace_crate1_config"],
    edition = "2024",
)

rust_library(
    name = "namespace_crate3",
    testonly = 1,
    srcs = ["namespace_crate3.rs"],
    edition = "2024",
)

rust_library(
    name = "namespace_crate2",
    testonly = 1,
    srcs = ["namespace_crate2.rs"],
    aspect_hints = [":namespace_crate2_cc_api.hint"],
    edition = "2024",
    deps = [
        ":namespace_crate1",
        ":namespace_crate3",
    ],
)

cpp_api_from_rust(
    name = "namespace_crate2_cc_api",
    testonly = 1,
    crate = ":namespace_crate2",
    namespace = "crate2_namespace::crate2_subnamespace",
)

crubit_cc_test(
    name = "namespace_test",
    srcs = ["namespace_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":namespace_crate2_cc_api",
        "@googletest//:gtest_main",
    ],
)

rust_library(
    name = "missing_hint_crate",
    srcs = ["namespace_crate1.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "missing_hint_crate_cc_api",
    crate = ":missing_hint_crate",
    namespace = "some::namespace",
    tags = ["manual"],
)

missing_hint_test(
    name = "missing_hint_test",
    target_under_test = ":missing_hint_crate_cc_api",
)

cpp_api_from_rust_library_config(
    name = "duplicate_hint_1",
    namespace = "ns1",
)

cpp_api_from_rust_library_config(
    name = "duplicate_hint_2",
    namespace = "ns2",
)

rust_library(
    name = "duplicate_hint_crate",
    srcs = ["namespace_crate1.rs"],
    aspect_hints = [
        ":duplicate_hint_1",
        ":duplicate_hint_2",
    ],
    edition = "2024",
)

cpp_api_from_rust(
    name = "duplicate_hint_crate_cc_api",
    crate = ":duplicate_hint_crate",
    tags = ["manual"],
)

duplicate_hint_test(
    name = "duplicate_hint_test",
    target_under_test = ":duplicate_hint_crate_cc_api",
)
