#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Checks that the bindings of `:lib` contain the ordinary `//` comments of
# `shared.h`. When header modules are used, these comments aren't in the
# precompiled header modules, so `rs_bindings_from_cc` reads them from the
# source of the headers instead.

set -euo pipefail

readonly RS_FILE="$1"
STATUS=0

# Compares lines without their leading whitespace. (This doesn't pipe into
# `grep -q`, which can make the pipeline fail with `pipefail` when `grep` exits
# early.)
expect_line() {
  if ! awk -v line="$1" '{ sub(/^ +/, "") } $0 == line { found = 1; exit }
                         END { exit !found }' "${RS_FILE}"; then
    echo >&2 "Missing line: '$1'"
    STATUS=1
  fi
}

# Doc comments of a class and its member function.
expect_line "/// Ordinary \`//\` comments are used as doc comments, even though precompiled"
expect_line "/// header modules don't contain them."
expect_line "/// Returns the value."
# Free comments.
expect_line "// A free comment, which isn't attached to any declaration."
expect_line "// namespace shared_ns"

if (( STATUS != 0 )); then
  echo >&2 "In ${RS_FILE}:"
  cat >&2 "${RS_FILE}"
fi
exit "${STATUS}"
