#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


source gbash.sh || exit

set -euo pipefail

RUST_LIBRARIES="${RUNFILES}/google3/third_party/unsupported_toolchains/rust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib"
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}${RUST_LIBRARIES}"

exec "${RUNFILES}/cc_bindings_from_rs/cc_bindings_from_rs" "${@}"