load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)

rust_proc_macro(
    name = "cc_template",
    srcs = ["cc_template.rs"],
    visibility = ["//visibility:public"],
    deps = [
        ":cc_template_impl",
        "@crates_io//syn/v1:syn",
    ],
)

rust_library(
    name = "cc_template_impl",
    srcs = ["cc_template_impl.rs"],
    deps = [
        "//common:token_stream_printer",
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//proc_macro2/v1:proc_macro2",
        "@crates_io//quote/v1:quote",
        "@crates_io//serde_json/v1:serde_json",
        "@crates_io//syn/v1:syn",
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
        "@crates_io//itertools/v0_10:itertools",
        "@crates_io//maplit/v1:maplit",
    ],
)
