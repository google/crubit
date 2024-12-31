load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

crubit_test_cc_library(
    name = "unsafe_attributes",
    srcs = ["unsafe_attributes.cc"],
    hdrs = ["unsafe_attributes.h"],
    deps = [
        "//support:annotations",
    ],
)

cc_library(
    name = "example_lib",
    hdrs = ["example.h"],

    # Opt into using Crubit.
    aspect_hints = ["//features:supported"],
    deps = [
        "//support:annotations",
    ],
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],

    # Declare a dependency on Rust bindings for calling into the C++ `example_lib` library:
    cc_deps = [":example_lib"],
)

# This test rule is just to make sure we don't forget to keep the `example_generated.rs` updated!
golden_test(
    name = "example_golden_test",
    cc_library = "example_lib",
    golden_rs = "example_generated.rs",
)
