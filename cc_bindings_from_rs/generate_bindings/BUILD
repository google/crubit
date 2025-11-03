"""Generates C++ bindings from Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

rust_library(
    name = "generate_bindings",
    srcs = [
        "format_type.rs",
        "generate_function.rs",
        "generate_function_thunk.rs",
        "generate_struct_and_union.rs",
        "lib.rs",
    ],
    proc_macro_deps = [
        "@crate_index//:rustversion",  # v1
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr,cfg_accessible,stmt_expr_attributes,proc_macro_hygiene"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":query_compiler",
        "//cc_bindings_from_rs:crubit_attr",
        "//cc_bindings_from_rs:run_compiler",
        "//cc_bindings_from_rs:toposort",
        "//cc_bindings_from_rs/generate_bindings/database",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_abi_type",
        "//common:crubit_feature",
        "//common:dyn_format",
        "//common:error_report",
        "@crate_index//:flagset",  # v0_4
        "@crate_index//:heck",  # v0_5
        "@crate_index//:itertools",  # v0_13
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
        "@crate_index//:syn",  # v1
    ],
)

crubit_rust_test(
    name = "format_type_test",
    srcs = ["format_type_test.rs"],
    rustc_flags = ["-Zallow-features=rustc_private"],
    deps = [
        ":generate_bindings",
        ":test_helpers",
        "//cc_bindings_from_rs:run_compiler_test_support",
        "//cc_bindings_from_rs/generate_bindings/database",
        "//common:code_gen_utils",
        "//common:token_stream_matchers",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
    ],
)

crubit_rust_test(
    name = "generate_bindings_test",
    srcs = ["generate_bindings_test.rs"],
    rustc_flags = ["-Zallow-features=rustc_private"],
    deps = [
        ":generate_bindings",
        ":test_helpers",
        "//cc_bindings_from_rs:run_compiler_test_support",
        "//common:code_gen_utils",
        "//common:token_stream_matchers",
        "//common:token_stream_printer",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
    ],
)

crubit_rust_test(
    name = "generate_function_test",
    srcs = ["generate_function_test.rs"],
    deps = [
        ":test_helpers",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:token_stream_matchers",
        "@crate_index//:flagset",  # v0_4
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
    ],
)

crubit_rust_test(
    name = "generate_struct_and_union_test",
    srcs = ["generate_struct_and_union_test.rs"],
    deps = [
        ":test_helpers",
        "//common:token_stream_matchers",
        "@crate_index//:quote",  # v1
    ],
)

rust_library(
    name = "test_helpers",
    testonly = True,
    srcs = [
        "test_helpers.rs",
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr,cfg_accessible,stmt_expr_attributes,proc_macro_hygiene"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    tags = [
        "manual",
    ],
    deps = [
        ":generate_bindings",
        "//cc_bindings_from_rs:run_compiler_test_support",
        "//cc_bindings_from_rs/generate_bindings/database",
        "//common:arc_anyhow",
        "//common:crubit_feature",
        "//common:dyn_format",
        "//common:error_report",
        "@crate_index//:flagset",  # v0_4
    ],
)

rust_library(
    name = "query_compiler",
    srcs = [
        "query_compiler.rs",
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "//common:arc_anyhow",
        "//common:error_report",
    ],
)
