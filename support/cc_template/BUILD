load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)

package(default_applicable_licenses = ["//:license"])

rust_proc_macro(
    name = "cc_template",
    srcs = ["cc_template.rs"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":cc_template_impl",
        "@crate_index//:syn",
    ],
)

rust_library(
    name = "cc_template_impl",
    srcs = ["cc_template_impl.rs"],
    deps = [
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "cc_template_test",
    args = [
        # To prevent tests from polluting the environment variables for each other.
        "--test-threads",
        "1",
    ],
    crate = ":cc_template_impl",
    deps = [
        "@crate_index//:maplit",
    ],
)
