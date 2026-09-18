#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Args: $1 = stable_nonprod binary, $2 = expected supported features, $3 = excluded features.

# 1. Fail if stable_nonprod rejects any feature in SUPPORTED_FEATURES ($2).
output=$("$1" --default-features="$2" 2>&1 || true)
if [[ "$output" != *"the following required arguments were not provided"* ]]; then
  echo "$output" >&2
  if [[ "$output" == *"Invalid Crubit feature name"* ]]; then
    echo "ERROR: Please add the unsupported feature above to NONPROD_UNSUPPORTED_FEATURES in //features/global_features.bzl" >&2
  else
    echo "ERROR: Unexpected failure when invoking stable_nonprod cc_bindings_from_rs binary." >&2
  fi
  exit 1
fi

# 2. Warn (non-blocking) if any feature in NONPROD_UNSUPPORTED_FEATURES ($3) is now supported.
for feat in ${3//,/ }; do
  feat_output=$("$1" --default-features="$feat" 2>&1 || true)
  if [[ "$feat_output" == *"the following required arguments were not provided"* ]]; then
    echo "WARNING: '$feat' is now supported by stable_nonprod and can be removed from NONPROD_UNSUPPORTED_FEATURES." >&2
  fi
done

echo "PASS: All non-excluded SUPPORTED_FEATURES ($2) are recognized by stable_nonprod."
