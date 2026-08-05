load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

licenses(["notice"])

cc_library(
    name = "example_lib",
    hdrs = ["example.h"],

    # Opt into using Crubit.
    aspect_hints = [":example_lib_rust.hint"],
)

rust_api_from_cpp(
    name = "example_lib_rust",
    cpp_target = ":example_lib",
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],

    # Declare a dependency on Rust bindings for calling into the C++ `example_lib` library:
    deps = [":example_lib_rust"],
)

# This test rule is just to make sure we don't forget to keep the `example_generated.rs` updated!
golden_test(
    name = "example_golden_test",
    cc_library = "example_lib",
    golden_rs = "example_generated.rs",
)

# This BUILD rule is here for exposition only. This is an example of what happens if
# you _don't_ set things up for Crubit.
cc_library(
    name = "example_lib_broken",
    hdrs = ["example.h"],
)
