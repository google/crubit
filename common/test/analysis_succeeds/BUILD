load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common/test/analysis_succeeds:transition.bzl", "unsupported_by_crubit")

package(
    default_applicable_licenses = ["//:license"],
)

platform(
    name = "unsupported_by_crubit_platform",
    constraint_values = [
        # Doesn't matter what we pick, as long as it isn't a supported platform for Crubit
        # but is a supported platform for C++, Rust.
        "@platforms//os:android",
        "@platforms//cpu:aarch64",
    ],
)

cc_library(
    name = "cc_lib",
    hdrs = ["cc_lib.h"],
)

rust_library(
    name = "rust_lib",
    srcs = ["rust_lib.rs"],
    cc_deps = [":cc_lib"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "rust_lib_cc",
    crate = ":rust_lib",
)

unsupported_by_crubit(
    name = "analysis_succeeds",
    aspect_hints = [":rust_lib_cc"],
)
