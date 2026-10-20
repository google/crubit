load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(
    default_applicable_licenses = ["//:license"],
)

cc_library(
    name = "base",
    hdrs = ["base.h"],

    # Opt into using Crubit.
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "base_rust",
    cpp_target = ":base",
)

cc_library(
    name = "example_lib",
    hdrs = ["example.h"],

    # Opt into using Crubit.
    aspect_hints = ["//features:supported"],
    deps = [
        ":base",
        ":definition_cc",
    ],
)

rust_api_from_cpp(
    name = "example_lib_rust",
    cpp_target = ":example_lib",
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],
    edition = "2024",

    # Declare a dependency on Rust bindings for calling into the C++ `example_lib` library:
    deps = [
        ":base_rust",
        ":definition",
        ":example_lib_rust",
        "//support:ctor",
    ],
)

# This test rule is just to make sure we don't forget to keep the `example_generated.rs` updated!
golden_test(
    name = "example_golden_test",
    cc_library = "example_lib",
    golden_rs = "example_generated.rs",
)

rust_library(
    name = "definition",
    srcs = ["definition.rs"],
    edition = "2024",
    deps = [
        ":base_rust",
        "//support/ffi_11",
    ],
)

cpp_api_from_rust(
    name = "definition_cc",
    crate = ":definition",
)
