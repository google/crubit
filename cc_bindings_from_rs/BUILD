"""Generates C++ bindings from Rust APIs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

licenses(["notice"])

# The current Rust toolchain uses shared libraries for rustc_driver and friends, and we don't have
# easily usable support for setting up the environment for shared libraries in Bazel. Either:
# * use `:cc_bindings_from_rs_legacy_toolchain_runner`
# * use the new toolchain by passing
#   `--extra_toolchains=//nowhere/llvm/rust:rust_x86_64` to Bazel.
# TODO(b/262583967): Remove legacy_toolchain support scripts once we roll out the bazel-built toolchain.
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
    experimental_use_cc_common_link = 0,
    # TODO(b/262583967): Remove once we roll out the bazel-built toolchain.
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
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

# TODO(b/262583967): Remove once we statically link rustc_driver.
sh_binary(
    name = "cc_bindings_from_rs_legacy_toolchain_runner",
    srcs = ["cc_bindings_from_rs_legacy_toolchain_runner.sh"],
    bash_version = "system",
    data = [
        ":cc_bindings_from_rs",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:rustc_driver_libs",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:stdlibs_generated",
    ],
    visibility = ["//:__subpackages__"],
    deps = ["//util/shell/gbash"],
)

rust_test(
    name = "cc_bindings_from_rs_test",
    crate = ":cc_bindings_from_rs",
    data = [
        "//nowhere/llvm/rust:std_libs",
    ],
    # TODO(b/242703401): Remove once cc_common.link works for rustc libraries.
    experimental_use_cc_common_link = 0,
    # TODO(hlopko): Remove once we roll out the bazel-built toolchain.
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        "//common:token_stream_matchers",
        "@crate_index//:regex",
        "@crate_index//:tempfile",
    ],
)

# TODO(b/262583967): Remove once we statically link rustc_driver.
sh_test(
    name = "cc_bindings_from_rs_legacy_toolchain_test",
    srcs = ["legacy_toolchain_test_wrapper.sh"],
    data = [
        ":cc_bindings_from_rs_test",
        "//nowhere/llvm/rust:main_sysroot/bin/rustfmt",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:rustc_driver_libs",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:stdlibs_generated",
    ],
    env = {"LEGACY_TOOLCHAIN_RUST_TEST": "cc_bindings_from_rs_test"},
)

sh_test(
    name = "cc_bindings_from_rs_sh_test",
    srcs = ["cc_bindings_from_rs_sh_test.sh"],
    data = [
        # TODO(b/262583967): This should directly depend on the
        # `cc_bindings_from_rs` binary (without the indirection of `.sar` or
        # `sh_binary`).  This can be done once we statically link rustc_driver.
        ":cc_bindings_from_rs_legacy_toolchain_runner.sar",
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
