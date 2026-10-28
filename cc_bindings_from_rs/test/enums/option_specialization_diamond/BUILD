"""End-to-end tests of `cc_bindings_from_rs`, focusing on enums-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "foo",
    testonly = 1,
    srcs = ["foo.rs"],
    edition = "2024",
)

rust_library(
    name = "bar",
    testonly = 1,
    srcs = ["bar.rs"],
    edition = "2024",
)

rust_library(
    name = "foobar",
    testonly = 1,
    srcs = ["foobar.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        ":bar",
        ":foo",
    ],
)

cpp_api_from_rust(
    name = "foobar_cc_api",
    testonly = 1,
    crate = ":foobar",
)

crubit_cc_test(
    name = "option_specialization_diamond",
    srcs = ["foobar_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":foobar_cc_api",
        "@googletest//:gtest_main",
    ],
)
