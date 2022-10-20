#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

source gbash.sh || exit

set -euo pipefail

# `cc_bindings_from_rs` needs extra help with finding Rust stdlib.  This is
# accomplished by passing `--sysroot "${RUST_SYSROOT}"` on the cmdline - see:
# https://doc.rust-lang.org/rustc/command-line-arguments.html#--sysroot-override-the-system-root
#
# Without explicit setting of `--sysroot`, the following errors would be
# reported by the Rust compiler when running the `cc_bindings_from_rs` tool:
# error[E0463]: can't find crate for `std`
RUST_SYSROOT="${RUNFILES}/google3/third_party/unsupported_toolchains/rust/toolchains/nightly"

# LD_LIBRARY_PATH is set as a temporary workaround for dynamic linking
# of `lib/librustc_driver-*.so`.
#
# TODO(b/242703401): Remove once `rustc_driver` and other Rust compiler code is
# linked statically into `cc_bindings_from_rs`.
RUST_LIBRARIES="${RUST_SYSROOT}/lib/rustlib/x86_64-unknown-linux-gnu/lib"
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}${RUST_LIBRARIES}"

exec "${RUNFILES}/cc_bindings_from_rs/cc_bindings_from_rs" "${@}" \
  --sysroot "${RUST_SYSROOT}"
