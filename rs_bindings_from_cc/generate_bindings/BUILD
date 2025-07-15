load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)
load(
    "//common:multiplatform_testing.bzl",
    "multiplatform_rust_test",
)

rust_library(
    name = "cc_api",
    srcs = [
        "cc_api.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":generate_bindings",
        "//common:error_report",
        "//common:ffi_types",
        "//rs_bindings_from_cc/generate_bindings/database",
    ],
)

rust_library(
    name = "generate_bindings",
    srcs = [
        "lib.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":cpp_type_name",
        ":generate_comment",
        ":generate_enum",
        ":generate_function",
        ":generate_function_thunk",
        ":generate_struct_and_union",
        ":has_bindings",
        ":rs_type_kind",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_abi_type",
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
    srcs = ["generate_bindings_test.rs"],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":multiplatform_ir_testing",
        ":test_generators",
        "//common:arc_anyhow",
        "//common:crubit_feature",
        "//common:multiplatform_testing",
        "//common:token_stream_matchers",
        "//common:token_stream_printer",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc:ir_matchers",
        "//rs_bindings_from_cc:ir_testing",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:googletest",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:static_assertions",
        "@crate_index//:syn",
    ],
)

rust_library(
    name = "cpp_type_name",
    srcs = ["cpp_type_name.rs"],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:error_report",
        "//rs_bindings_from_cc:ir",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

rust_library(
    name = "generate_struct_and_union",
    srcs = [
        "generate_struct_and_union.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":cpp_type_name",
        ":generate_comment",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

multiplatform_rust_test(
    name = "generate_struct_and_union_test",
    srcs = ["generate_struct_and_union_test.rs"],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":generate_bindings",
        ":generate_struct_and_union",
        ":multiplatform_ir_testing",
        ":test_generators",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:multiplatform_testing",
        "//common:token_stream_matchers",
        "//common:token_stream_printer",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc:ir_matchers",
        "//rs_bindings_from_cc:ir_testing",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:googletest",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:static_assertions",
        "@crate_index//:syn",
    ],
)

rust_library(
    name = "multiplatform_ir_testing",
    testonly = True,
    srcs = ["multiplatform_ir_testing.rs"],
    visibility = ["//rs_bindings_from_cc:__subpackages__"],
    deps = [
        "//common:arc_anyhow",
        "//common:multiplatform_testing",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc:ir_testing",
    ],
)

rust_library(
    name = "test_generators",
    testonly = True,
    srcs = ["test_generators.rs"],
    visibility = ["//rs_bindings_from_cc:__subpackages__"],
    deps = [
        ":generate_bindings",
        ":multiplatform_ir_testing",
        "//common:arc_anyhow",
        "//common:error_report",
        "//common:ffi_types",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
    ],
)

rust_library(
    name = "generate_comment",
    srcs = [
        "generate_comment.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:ffi_types",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

rust_test(
    name = "generate_comment_test",
    srcs = ["generate_comment_test.rs"],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":generate_bindings",
        ":generate_comment",
        "//common:arc_anyhow",
        "//common:error_report",
        "//common:ffi_types",
        "//common:token_stream_matchers",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc:ir_matchers",
        "//rs_bindings_from_cc:ir_testing",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:googletest",
        "@crate_index//:quote",
    ],
)

rust_library(
    name = "generate_enum",
    srcs = [
        "generate_enum.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":generate_comment",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:error_report",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

multiplatform_rust_test(
    name = "generate_enum_test",
    srcs = ["generate_enum_test.rs"],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":generate_bindings",
        ":multiplatform_ir_testing",
        ":test_generators",
        "//common:arc_anyhow",
        "//common:token_stream_matchers",
        "@crate_index//:googletest",
        "@crate_index//:quote",
    ],
)

rust_library(
    name = "generate_function_thunk",
    srcs = [
        "generate_function_thunk.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":cpp_type_name",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_abi_type",
        "//common:error_report",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
        "@crate_index//:unicode-ident",
    ],
)

rust_library(
    name = "generate_function",
    srcs = [
        "generate_function.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":generate_comment",
        ":generate_function_thunk",
        ":has_bindings",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_abi_type",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:errors",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

multiplatform_rust_test(
    name = "generate_function_test",
    srcs = ["generate_function_test.rs"],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":generate_bindings",
        ":generate_function_thunk",
        ":multiplatform_ir_testing",
        ":test_generators",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:multiplatform_testing",
        "//common:token_stream_matchers",
        "//common:token_stream_printer",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc:ir_matchers",
        "//rs_bindings_from_cc:ir_testing",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:googletest",
        "@crate_index//:quote",
        "@crate_index//:static_assertions",
        "@crate_index//:syn",
    ],
)

rust_library(
    name = "has_bindings",
    srcs = [
        "has_bindings.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:crubit_feature",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
        "@crate_index//:heck",
    ],
)

rust_library(
    name = "rs_type_kind",
    srcs = [
        "rs_type_kind.rs",
    ],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        ":cpp_type_name",
        ":has_bindings",
        "//common:arc_anyhow",
        "//rs_bindings_from_cc:ir",
        "//rs_bindings_from_cc/generate_bindings/database",
    ],
)
