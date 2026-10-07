load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
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

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

# This declares an "example_crate_cc_api" target that provides Crubit-generated
# C++ bindings for the Rust crate behind the `":example_crate"` target.
rust_library(
    name = "example_crate",
    srcs = ["example.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "example_crate_cc_api",
    crate = ":example_crate",
)

cc_binary(
    name = "main",
    srcs = ["main.cc"],
    deps = [
        ":example_crate_cc_api",
        "//support/rs_std:rs_alloc",
        "//support/rs_std:str_ref",
    ],
)

golden_test(
    name = "example_golden_test",
    golden_h = "example_generated.h",
    rust_library = "example_crate",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)
