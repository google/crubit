# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Paths to external binaries used by Crubit.

In all likelihood this should become a toolchain, or else we should find a way to extract it from
the existing C++/Rust toolchains. But for now, at least, we centralize the locations here.
"""

# Note: this has different paths in the Google monorepo.
EXTERNAL_BINARIES = {
       "CRUBIT_RUSTFMT_EXE_PATH": "../+crubit_toolchains+rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain/bin/rustfmt",
       "CRUBIT_CLANG_FORMAT_EXE_PATH": "../+crubit_toolchains+llvm_toolchain/clang-format",
}
