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
        "generate_comment.rs",
        "generate_enum.rs",
        "generate_function.rs",
        "generate_function_thunk.rs",
        "generate_struct_and_union.rs",
        "lib.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:errors",
        "//common:ffi_types",
        "//common:token_stream_printer",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
        "@crate_index//:unicode-ident",
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
