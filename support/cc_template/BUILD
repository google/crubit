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
        "@crates//:syn",  # v1
    ],
)

rust_library(
    name = "cc_template_impl",
    srcs = ["cc_template_impl.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    deps = [
        "@crates//:anyhow",  # v1
        "@crates//:proc-macro2",
        "@crates//:quote",  # v1
        "@crates//:serde_json",  # v1
        "@crates//:syn",  # v1
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
        "@crates//:googletest",
        "@crates//:maplit",  # v1
    ],
)
