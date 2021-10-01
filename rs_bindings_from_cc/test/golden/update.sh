#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


set -euo pipefail

if [[ "${PWD}" != *'/google3'* ]]; then
  echo "Usage: Please run this script from within a client."
  exit 1
fi

G3="${PWD%%/google3*}/google3"
PKG="rs_bindings_from_cc/test/golden"

cd "${G3}/${PKG}"

TARGETS=()
for header in *.h; do
  TARGETS+=(":${header%.h}_cc_file")
  TARGETS+=(":${header%.h}_rs_file")
done

bazel build "${TARGETS[@]}"

for header in *.h; do
  cat "$(bazel info bazel-bin)/${PKG}/${header%.h}_bindings_files_rs_api.rs" >"${header%.h}_rs_api.rs"
  cat "$(bazel info bazel-bin)/${PKG}/${header%.h}_bindings_files_rs_api_impl.cc" >"${header%.h}_rs_api_impl.cc"
done
