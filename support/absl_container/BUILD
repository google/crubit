"""Support libraries for the generated Rust APIs of Abseil container C++ types."""

load("@rules_rust//rust:defs.bzl", "rust_library")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "absl_container",
    srcs = ["absl_container.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
)
