load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

rust_proc_macro(
    name = "cc_template",
    srcs = ["cc_template.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":cc_template_impl",
        "@crate_index//:syn",  # v1
    ],
)

rust_library(
    name = "cc_template_impl",
    srcs = ["cc_template_impl.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2021",
    deps = [
        "@crate_index//:anyhow",  # v1
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
        "@crate_index//:serde_json",  # v1
        "@crate_index//:syn",  # v1
    ],
)

crubit_rust_test(
    name = "cc_template_test",
    args = [
        # To prevent tests from polluting the environment variables for each other.
        "--test-threads",
        "1",
    ],
    crate = ":cc_template_impl",
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:maplit",  # v1
    ],
)
