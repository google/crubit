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
HEADERS_TO_SKIP=(
  # The namespaces_json.h header is used to inspect the generated .json file.
  # Updates to namespaces_json.json, if needed, shall be performed manually.
  "namespaces_json.h"
)

for header in *.h; do
  if [[ "${HEADERS_TO_SKIP[@]}" =~ "${header}" ]]; then
    continue;
  fi
  TARGETS+=(":${header%.h}_cc_file")
  TARGETS+=(":${header%.h}_rs_file")
done

bazel build "${TARGETS[@]}"

for header in *.h; do
  if [[ "${HEADERS_TO_SKIP[@]}" =~ "${header}" ]]; then
    continue;
  fi
  # Since these files are checked in, they need a license header.
  cat LICENSE_HEADER "$(bazel info bazel-bin)/${PKG}/${header%.h}_cc_rust_api.rs" > "${header%.h}_rs_api.rs"
  cat LICENSE_HEADER "$(bazel info bazel-bin)/${PKG}/${header%.h}_cc_rust_api_impl.cc" > "${header%.h}_rs_api_impl.cc"
done
