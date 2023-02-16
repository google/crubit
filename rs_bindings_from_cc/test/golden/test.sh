#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

function prepend_license() {
  cat rs_bindings_from_cc/test/golden/LICENSE_HEADER "$1"
}

diff -u -I '// Generated from: ' -I '// Expanded at: ' "$1" <(prepend_license "$2")
STATUS1=$?

if [ "$#" == 2 ]; then
  STATUS2=0
else
  diff -u -I '// Generated from: ' -I '// Expanded at: ' "$3" <(prepend_license "$4")
  STATUS2=$?
fi

if (($STATUS1 != 0 || $STATUS2 != 0)); then
  exit 1
fi
