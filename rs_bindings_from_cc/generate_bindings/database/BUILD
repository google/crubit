load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:multiplatform_testing.bzl",
    "multiplatform_rust_test",
)

rust_library(
    name = "database",
    srcs = [
        "code_snippet.rs",
        "db.rs",
        "function_types.rs",
        "lib.rs",
        "rs_snippet.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_abi_type",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:errors",
        "//common:ffi_types",
        "//common:memoized",
        "//common:token_stream_printer",
        "//rs_bindings_from_cc:ir",
        "@crate_index//:flagset",
        "@crate_index//:heck",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

multiplatform_rust_test(
    name = "database_test",
    crate = ":database",
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        "//common:multiplatform_testing",
        "//common:token_stream_matchers",
        "//rs_bindings_from_cc:ir_matchers",
        "@crate_index//:googletest",
    ],
)
