# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")


_rust_toolchain_dir_build_file_contents = """
package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:toolchain.bzl", "rust_toolchain")

rust_stdlib_filegroup(
    name = "std_libs",
    srcs = glob(
        ["custom_rust/lib/rustlib/x86_64-unknown-linux-gnu/**/*" + f + "*.rlib" for f in [
            "addr2line",
            "adler",
            "alloc",
            "backtrace",
            "backtrace_sys",
            "cfg_if",
            "compiler_builtins",
            "core",
            "getopts",
            "gimli",
            "hashbrown",
            "libc",
            "memchr",
            "miniz_oxide",
            "object",
            "panic_abort",
            "panic_unwind",
            "proc_macro",
            "profiler_builtins",
            "rustc_demangle",
            "std",
            "term",
            "test",
            "unicode_width",
            "unwind",
        ]],
    ) + [
        # DO NOT SUBMIT - remove me
        # libclang_rt.builtins.a
        # libclang_rt.asan_static.a
    ],
)

filegroup(
    name = "compiler_files",
    srcs = glob([
        "custom_rust/lib/rustlib/**",
    ]) + ["bin/rustc"],
)

rust_toolchain(
    name = "custom_rust_toolchain_impl",
    allocator_library = "do-not-submit/rust/support:remap_alloc", # DO NOT SUBMIT / FIXME
    binary_ext = "",
    clippy_driver = ":custom_rust/bin/clippy-driver",
    default_edition = "2021",
    dylib_ext = ".so",
    exec_triple = "x86_64-unknown-linux-gnu",
    os = "linux",
    rust_doc = ":custom_rust/bin/rustdoc",
    rust_std = ":std_libs",
    rustc = ":custom_rust/bin/rustc",
    rustc_lib = ":compiler_files",
    rustfmt = ":custom_rust/bin/rustfmt",
    staticlib_ext = ".a",
    stdlib_linkflags = ["-lpthread", "-ldl"],
    target_triple = "x86_64-unknown-linux-gnu",
)

# DO NOT SUBMIT / FIXME
#
# The `toolchain` line is based on https://docs.bazel.build/versions/0.19.1/toolchains.html#creating-a-toolchain-definition
#
# But... apparently this doesn't work well with the usage of `register_toolchains`
# in WORKSPACE...
#
# ERROR: /usr/local/google/home/lukasza/src/chromium4/src/third_party/crubit/rs_bindings_from_cc/BUILD:62:10: While resolving toolchains for target //rs_bindings_from_cc:rs_bindings_from_cc_impl: invalid registered toolchain '@custom_rust_toolchain_repo//:custom_rust_toolchain': no such target '@custom_rust_toolchain_repo//:custom_rust_toolchain': target 'custom_rust_toolchain' not declared in package '' defined by /usr/local/google/home/lukasza/.cache/bazel/_bazel_lukasza/0aa27f9c6fcd7410d84c2f4a4fdcb7bc/external/custom_rust_toolchain_repo/BUILD

toolchain(
    name = "custom_rust_toolchain",
    exec_compatible_with = [
        "@bazel_tools//platforms:linux",
        "@bazel_tools//platforms:x86_64",
    ],
    target_compatible_with = [
        "@bazel_tools//platforms:linux",
        "@bazel_tools//platforms:x86_64",
    ],
    toolchain = ":custom_rust_toolchain_impl",
    toolchain_type = "@rules_rust//rust:toolchain",
)
"""


def _crubit_rust_toolchain_repository_impl(repository_ctx):
    path = repository_ctx.os.environ["RUST_TOOLCHAIN_PATH"]

    # If needed, resolve relative to root of *calling* repository
    if not path.startswith("/"):
        root_path = repository_ctx.path(
            repository_ctx.attr.file_at_root,
        ).dirname
        path = repository_ctx.path(str(root_path) + "/" + path)

    repository_ctx.symlink(path, "custom_rust")

    repository_ctx.file("BUILD", _rust_toolchain_dir_build_file_contents)


custom_rust_toolchain_repository = repository_rule(
    implementation = _crubit_rust_toolchain_repository_impl,
    attrs = {
        # We need a file from the root in order to get the workspace path
        "file_at_root": attr.label(default = "//:BUILD"),
    },
    environ = [
        "RUST_TOOLCHAIN_PATH",
    ],
)

