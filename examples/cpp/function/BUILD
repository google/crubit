load("@rules_rust//rust:defs.bzl", "rust_binary")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

cc_library(
    name = "example_lib",
    hdrs = ["example.h"],

    # Opting into using Crubit (including Crubit's experimental features).
    aspect_hints = ["//features:experimental"],
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],

    # Declaring a dependency on Rust bindings for calling into the C++
    # `example_lib` library:
    cc_deps = [":example_lib"],
)

# This test rule is just to make sure we don't forget to keep the `example_generated.rs` updated!
golden_test(
    name = "example_golden_test",
    cc_library = "example_lib",
    golden_rs = "example_generated.rs",
)
