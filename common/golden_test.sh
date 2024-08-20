#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

function prepend_license() {
  cat common/LICENSE_HEADER "$1"
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
  if [ $NEW_STATUS != 0 ]; then
    if [ -n "$WRITE_GOLDENS" ]; then
      prepend_license "$2" > "$1"
    elif [ $STATUS == 0 ]; then
      STATUS="$NEW_STATUS"
    fi
  fi
  shift 2
done

if [ $STATUS != 0 ]; then
  echo >&2 "To regenerate the goldens, run cc_bindings_from_rs/test/golden/update.sh"
  exit 1
fi