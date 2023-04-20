"""Generates C++ bindings from Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_binary(
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
    visibility = ["//visibility:public"],
    deps = [
        ":toposort",
        "//common:code_gen_utils",
        "//common:rust_allocator_shims",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:clap",
        "@crate_index//:either",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@rules_rust//tools/runfiles",
    ],
)

rust_test(
    name = "cc_bindings_from_rs_test",
    crate = ":cc_bindings_from_rs",
    data = select({
        "//third_party/crosstool:unstable_config": ["//nowhere/llvm/rust:std_libs"],
        "//conditions:default": ["//nowhere/llvm/rust:std_libs"],
    }),

    # TODO(b/242703401): Remove once cc_common.link works for rustc libraries.
    rustc_flags = select({
        "//third_party/crosstool:unstable_config": ["--cfg=llvm_unstable"],
        "//conditions:default": ["--cfg=stable"],
    }),
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

rust_test(
    name = "toposort_test",
    crate = ":toposort",
)
