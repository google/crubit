load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "merged_namespaces",
    srcs = ["merged_namespaces.rs"],
    deps = [
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

rust_test(
    name = "merged_namespaces_test",
    crate = "merged_namespaces",
)
