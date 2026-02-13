load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
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

rust_binary(
    name = "main",
    srcs = ["main.rs"],

    # Declare a dependency on Rust bindings for calling into the C++ `example_lib` library:
    cc_deps = [
        ":base",
        ":example_lib",
    ],
    deps = [
        ":definition",
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
    cc_deps = [
        ":base",
    ],
    deps = [
        "//support/ffi_11",
    ],
)

cc_bindings_from_rust(
    name = "definition_cc",
    crate = ":definition",
)
