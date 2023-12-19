#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

function prepend_license() {
  cat rs_bindings_from_cc/test/golden/LICENSE_HEADER "$1"
}

STATUS=0
while (("$#" != 0))
do
  if [ "$#" == 1 ]; then
    echo >&2 "INTERNAL ERROR: test.sh requires an even number of arguments."
    exit 1
  fi
  diff -u "$1" <(prepend_license "$2")
  NEW_STATUS="$?"
  if [ $STATUS == 0 ]; then
    STATUS="$NEW_STATUS"
  fi
  shift 2
done

if (($STATUS != 0)); then
  echo >&2 "To regenerate the goldens, run rs_bindings_from_cc/test/golden/update.sh"
  exit 1
fi
