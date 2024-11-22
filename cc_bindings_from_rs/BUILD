"""Generates C++ bindings from Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_binary",
    "crubit_rust_test",
    "crubit_sh_test",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

rust_library(
    name = "bindings",
    srcs = ["bindings.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":crubit_attr",
        ":run_compiler",
        ":toposort",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:memoized",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

crubit_rust_test(
    name = "bindings_test",
    crate = ":bindings",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,rustc_attr"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":run_compiler_test_support",
        "//common:token_stream_matchers",
        "@crate_index//:itertools",
    ],
)

crubit_rust_binary(
    name = "cc_bindings_from_rs",
    srcs = ["cc_bindings_from_rs.rs"],
    # Do not add a fake dependency on Crubit, because it will introduce a dependency cycle.
    crubit_dep = False,
    # TODO(b/242703401): Remove once cc_common.link works for rustc libraries.
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=never_type,rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":bindings",
        ":cmdline",
        ":run_compiler",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:token_stream_printer",
        "@crate_index//:clap",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
    ],
)

crubit_rust_test(
    name = "cc_bindings_from_rs_test",
    crate = ":cc_bindings_from_rs",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=never_type,rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":run_compiler_test_support",
        "@crate_index//:regex",
        "@crate_index//:tempfile",
    ],
)

crubit_sh_test(
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
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "//common:crubit_feature",
        "@crate_index//:anyhow",
        "@crate_index//:clap",
        "@crate_index//:flagset",
    ],
)

crubit_rust_test(
    name = "cmdline_test",
    crate = ":cmdline",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":run_compiler_test_support",
        "@crate_index//:itertools",
        "@crate_index//:tempfile",
    ],
)

rust_library(
    name = "crubit_attr",
    srcs = ["crubit_attr.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "@crate_index//:anyhow",
    ],
)

crubit_rust_test(
    name = "crubit_attr_test",
    crate = ":crubit_attr",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":run_compiler_test_support",
    ],
)

rust_library(
    name = "run_compiler",
    srcs = [
        "run_compiler.rs",
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "//common:arc_anyhow",
        "@crate_index//:either",
    ],
)

crubit_rust_test(
    name = "run_compiler_test",
    crate = ":run_compiler",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
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
    deps = [
        "@crate_index//:itertools",
        "@rules_rust//tools/runfiles",
    ],
)

crubit_rust_test(
    name = "run_compiler_test_support_test",
    crate = ":run_compiler_test_support",
    rustc_flags = ["--cfg=oss"],
)

rust_library(
    name = "toposort",
    srcs = ["toposort.rs"],
)

crubit_rust_test(
    name = "toposort_test",
    crate = ":toposort",
)
