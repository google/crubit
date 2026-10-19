#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Checks that two files of generated bindings are the same, except for the
# names of the targets that they were generated for.
#
# Usage: same_bindings_test.sh <file_ab> <file_ba>

set -euo pipefail

# Replaces the target names, and the hashes that disambiguate thunk names
# (which are computed from the name of the owning target).
normalize() {
  sed -E \
    -e 's/lib_(ab|ba)/lib_XX/g' \
    -e 's/__rust_thunk__[0-9a-f]{8}_/__rust_thunk__HASH_/g' \
    "$1"
}

if ! diff -u <(normalize "$1") <(normalize "$2"); then
  echo >&2 "The bindings of :lib_ab and :lib_ba differ (see the diff above)."
  exit 1
fi
