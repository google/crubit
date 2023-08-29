"""Generates C++ bindings from Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_binary",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_rust_binary(
    name = "cc_bindings_from_rs",
    srcs = [
        # TODO(b/254679226): These should be separate crates.
        "bindings.rs",
        "cc_bindings_from_rs.rs",
        "cmdline.rs",
        "run_compiler.rs",
    ],
    crate_root = "cc_bindings_from_rs.rs",
    # TODO(b/242703401): Remove once cc_common.link works for rustc libraries.
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":toposort",
        "//common:code_gen_utils",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:clap",
        "@crate_index//:either",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
        "@rules_rust//tools/runfiles",
    ],
)

crubit_rust_test(
    name = "cc_bindings_from_rs_test",
    crate = ":cc_bindings_from_rs",
    data = select({
        "//conditions:default": [
            "@rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools//:rust_std-x86_64-unknown-linux-gnu",
        ],
    }),
    # TODO(b/242703401): Remove once cc_common.link works for rustc libraries.
    rustc_flags = select({
        "//conditions:default": [
            "--cfg=oss",
        ],
    }),
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    deps = [
        "//common:token_stream_matchers",
        "@crate_index//:regex",
        "@crate_index//:tempfile",
    ],
)

sh_test(
    name = "cc_bindings_from_rs_sh_test",
    srcs = ["cc_bindings_from_rs_sh_test.sh"],
    data = [
        ":cc_bindings_from_rs",
    ],
    deps = [
        "//util/shell/gbash",
        "//util/shell/gbash:unit",
    ],
)

rust_library(
    name = "toposort",
    srcs = ["toposort.rs"],
)

crubit_rust_test(
    name = "toposort_test",
    crate = ":toposort",
)
