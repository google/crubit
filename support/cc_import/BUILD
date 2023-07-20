load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)

package(default_applicable_licenses = ["//:license"])

rust_proc_macro(
    name = "cc_import",
    srcs = [
        "cc_import.rs",
    ],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":cc_import_internal",
        "@crate_index//:proc-macro2",
        "@crate_index//:syn",
    ],
)

rust_library(
    name = "cc_import_internal",
    srcs = [
        "cc_import_internal.rs",
    ],
    deps = [
        ":merged_namespaces",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
        "@rules_rust//util/import:import_internal",
    ],
)

rust_library(
    name = "merged_namespaces",
    srcs = ["merged_namespaces.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
        "@rules_rust//util/import:import_internal",
    ],
)

rust_test(
    name = "merged_namespaces_test",
    crate = "merged_namespaces",
    deps = ["//common:token_stream_matchers"],
)
