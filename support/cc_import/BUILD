load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

rust_proc_macro(
    name = "cc_import",
    srcs = [
        "cc_import.rs",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":cc_import_internal",
        "@crates//:proc-macro2",
        "@crates//:syn",  # v1
    ],
)

rust_library(
    name = "cc_import_internal",
    srcs = [
        "cc_import_internal.rs",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    deps = [
        ":merged_namespaces",
        "@crates//:proc-macro2",
        "@crates//:quote",  # v1
        "@crates//:serde_json",  # v1
        "@crates//:syn",  # v1
        "@rules_rust//util/import:import_internal",
    ],
)

rust_library(
    name = "merged_namespaces",
    srcs = ["merged_namespaces.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    deps = [
        "@crates//:proc-macro2",
        "@crates//:quote",  # v1
        "@crates//:serde",  # v1
        "@crates//:serde_json",  # v1
        "@crates//:syn",  # v1
        "@rules_rust//util/import:import_internal",
    ],
)

crubit_rust_test(
    name = "merged_namespaces_test",
    crate = "merged_namespaces",
    deps = [
        "//common:token_stream_matchers",
        "@crates//:googletest",
    ],
)
