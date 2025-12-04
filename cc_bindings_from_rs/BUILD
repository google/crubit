"""Generates C++ bindings from Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//common/bazel_support:external_binaries.bzl", "EXTERNAL_BINARIES")
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

crubit_rust_binary(
    name = "cc_bindings_from_rs",
    srcs = ["cc_bindings_from_rs.rs"],
    # Do not add a fake dependency on Crubit, because it will introduce a dependency cycle.
    crubit_dep = False,
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=never_type,rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        ":cmdline",
        ":run_compiler",
        "//cc_bindings_from_rs/generate_bindings",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "//common:kythe_metadata",
        "//common:token_stream_printer",
        "@crate_index//:clap",  # v4
        "@crate_index//:flagset",  # v0_4
        "@crate_index//:itertools",  # v0_13
    ],
)

crubit_rust_test(
    name = "cc_bindings_from_rs_test",
    crate = ":cc_bindings_from_rs",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=never_type,rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    # We don't yet support running cc_bindings_from_rs itself on macOS (we only target macOS).
    tags = ["not_run:mac"],
    deps = [
        ":run_compiler_test_support",
        "//common:external_binaries",
        "@crate_index//:regex",  # v1
        "@crate_index//:tempfile",  # v3
    ],
)

crubit_sh_test(
    name = "cc_bindings_from_rs_sh_test",
    srcs = ["cc_bindings_from_rs_sh_test.sh"],
    data = [
        ":cc_bindings_from_rs",
    ],
    # We don't yet support running cc_bindings_from_rs itself on macOS (we only target macOS).
    tags = ["not_run:mac"],
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
        "//common:dyn_format",
        "@crate_index//:anyhow",  # v1
        "@crate_index//:clap",  # v4
        "@crate_index//:flagset",  # v0_4
    ],
)

crubit_rust_test(
    name = "cmdline_test",
    crate = ":cmdline",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    # We don't yet support running rustc itself on macOS (we only target macOS).
    tags = ["not_run:mac"],
    deps = [
        ":run_compiler_test_support",
        "@crate_index//:itertools",  # v0_13
        "@crate_index//:tempfile",  # v3
    ],
)

rust_library(
    name = "crubit_attr",
    srcs = ["crubit_attr.rs"],
    proc_macro_deps = [
        "@crate_index//:rustversion",  # v1
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "@crate_index//:anyhow",  # v1
    ],
)

crubit_rust_test(
    name = "crubit_attr_test",
    srcs = ["crubit_attr_test.rs"],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    # We don't yet support running rustc itself on macOS (we only target macOS).
    tags = ["not_run:mac"],
    deps = [
        ":crubit_attr",
        ":run_compiler_test_support",
        "@crate_index//:anyhow",  # v1
    ],
)

rust_library(
    name = "run_compiler",
    srcs = [
        "run_compiler.rs",
    ],
    proc_macro_deps = [
        "@crate_index//:rustversion",  # v1
    ],
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,cfg_accessible"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    deps = [
        "//common:arc_anyhow",
        "@crate_index//:either",  # v1
    ],
)

crubit_rust_test(
    name = "run_compiler_test",
    crate = ":run_compiler",
    # LINT.IfChange
    rustc_flags = ["-Zallow-features=rustc_private,cfg_accessible"],
    # LINT.ThenChange(//docs/overview/unstable_features.md)
    # We don't yet support running rustc itself on macOS (we only target macOS).
    tags = ["not_run:mac"],
    deps = [
        ":run_compiler_test_support",
        "@crate_index//:tempfile",  # v3
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
    proc_macro_deps = [
        "@crate_index//:rustversion",  # v1
    ],
    rustc_flags = [
        "--cfg=bazel",
        "--cfg=oss",
    ],
    deps = [
        "@crate_index//:itertools",  # v0_13
        "@rules_rust//tools/runfiles",
    ],
)

crubit_rust_test(
    name = "run_compiler_test_support_test",
    crate = ":run_compiler_test_support",
    rustc_flags = ["--cfg=oss"],
    # We don't yet support running rustc itself on macOS (we only target macOS).
    tags = ["not_run:mac"],
)

rust_library(
    name = "toposort",
    srcs = ["toposort.rs"],
)

crubit_rust_test(
    name = "toposort_test",
    crate = ":toposort",
)
