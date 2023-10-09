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

rust_library(
    name = "bindings",
    srcs = ["bindings.rs"],
    deps = [
        ":run_compiler",
        ":toposort",
        "//common:code_gen_utils",
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
        "@rules_rust//tools/runfiles",
    ],
)

crubit_rust_test(
    name = "bindings_test",
    crate = ":bindings",
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    deps = [
        ":run_compiler_test_support",
        "//common:token_stream_matchers",
        "@crate_index//:itertools",
    ],
)

crubit_rust_binary(
    name = "cc_bindings_from_rs",
    srcs = ["cc_bindings_from_rs.rs"],
    # TODO(b/242703401): Remove once cc_common.link works for rustc libraries.
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":bindings",
        ":cmdline",
        ":run_compiler",
        "//common:code_gen_utils",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:clap",
        "@crate_index//:itertools",
    ],
)

crubit_rust_test(
    name = "cc_bindings_from_rs_test",
    crate = ":cc_bindings_from_rs",
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    deps = [
        ":run_compiler_test_support",
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
    name = "cmdline",
    srcs = [
        "cmdline.rs",
    ],
    deps = [
        "@crate_index//:anyhow",
        "@crate_index//:clap",
    ],
)

crubit_rust_test(
    name = "cmdline_test",
    crate = ":cmdline",
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    deps = [
        ":run_compiler_test_support",
        "@crate_index//:itertools",
        "@crate_index//:tempfile",
    ],
)

rust_library(
    name = "run_compiler",
    srcs = [
        "run_compiler.rs",
    ],
    deps = [
        "@crate_index//:anyhow",
        "@crate_index//:either",
        "@crate_index//:once_cell",
    ],
)

crubit_rust_test(
    name = "run_compiler_test",
    crate = ":run_compiler",
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
    ],
    deps = [
        ":run_compiler_test_support",
        "@crate_index//:tempfile",
    ],
)

rust_library(
    name = "run_compiler_test_support",
    testonly = True,
    srcs = [
        "run_compiler_test_support.rs",
    ],
    data = [
        "@rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools//:rust_std-x86_64-unknown-linux-gnu",
    ],
    rustc_flags = ["--cfg=oss"],
    deps = ["@rules_rust//tools/runfiles"],
)

crubit_rust_test(
    name = "run_compiler_test_support_test",
    crate = ":run_compiler_test_support",
    rustc_flags = ["--cfg=oss"],
    tags = [
        "not_build:arm",
        "noubsan",  # rustc-as-a-library isn't supported for UBSan.
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
