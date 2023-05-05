""" Minimal example of Rust bindings for C++ APIs."""

load("@rules_rust//rust:defs.bzl", "rust_binary")

cc_library(
    name = "example_lib",
    srcs = ["example.cc"],
    hdrs = ["example.h"],

    # Opting into using Crubit (including Crubit's experimental features).
    aspect_hints = ["//:experimental"],
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],

    # Declaring a dependency on Rust bindings for calling into the C++
    # `example_lib` library:
    cc_deps = [":example_lib"],
)
