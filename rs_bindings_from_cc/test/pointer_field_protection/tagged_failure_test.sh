#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

set -euo pipefail

readonly RS_BINDINGS_FROM_CC="$1"
readonly HDR="$2"

readonly RS_OUT="${TEST_TMPDIR}/rs_api.rs"
readonly CC_OUT="${TEST_TMPDIR}/rs_api_impl.cc"
readonly STDERR="${TEST_TMPDIR}/stderr.txt"
readonly JSON="[{\"t\": \"//foo/bar:baz\", \"h\": [\"${HDR}\"], \"f\": [\"supported\"]}]"

if "${RS_BINDINGS_FROM_CC}" \
    --target=//foo/bar:baz \
    --rs_out="${RS_OUT}" \
    --cc_out="${CC_OUT}" \
    --crubit_support_path_format="<test/crubit/support/path/{header}>" \
    --public_headers="${HDR}" \
    --target_args="${JSON}" \
    -- -target aarch64-linux-gnu \
       -fexperimental-pointer-field-protection-abi \
       -fexperimental-pointer-field-protection-tagged \
    2>"${STDERR}"; then
  echo "Expected rs_bindings_from_cc to fail with tagged PFP" >&2
  exit 1
fi

if [[ -e "${RS_OUT}" || -e "${CC_OUT}" ]]; then
  echo "Expected no output files when tagged PFP fails" >&2
  exit 1
fi

if ! grep -q "Crubit does not support -fexperimental-pointer-field-protection-tagged" "${STDERR}"; then
  echo "Expected tagged PFP error message, got:" >&2
  cat "${STDERR}" >&2
  exit 1
fi
