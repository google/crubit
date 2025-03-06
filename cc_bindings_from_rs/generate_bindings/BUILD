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
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr,cfg_accessible"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":query_compiler",
        "//cc_bindings_from_rs:crubit_attr",
        "//cc_bindings_from_rs:run_compiler",
        "//cc_bindings_from_rs:toposort",
        "//cc_bindings_from_rs/generate_bindings/database",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

crubit_rust_test(
    name = "bindings_test",
    crate = ":generate_bindings",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr,cfg_accessible"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "//cc_bindings_from_rs:run_compiler_test_support",
        "//common:token_stream_matchers",
        "@crate_index//:itertools",
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
