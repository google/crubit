load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:multiplatform_testing.bzl",
    "multiplatform_rust_test",
)

rust_library(
    name = "generate_bindings",
    srcs = [
        "db.rs",
        "generate_func.rs",
        "generate_record.rs",
        "lib.rs",
        "rs_snippet.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:ffi_types",
        "//common:memoized",
        "//common:token_stream_printer",
        "//rs_bindings_from_cc:ir",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

multiplatform_rust_test(
    name = "generate_bindings_test",
    crate = ":generate_bindings",
    tags = ["not_run:arm"],
    deps = [
        "//common:multiplatform_testing",
        "//common:token_stream_matchers",
        "//rs_bindings_from_cc:ir_matchers",
        "//rs_bindings_from_cc:ir_testing",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)
