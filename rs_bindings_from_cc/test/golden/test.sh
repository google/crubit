#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


diff -u "$1" "$2"
STATUS1=$?

diff -u "$3" "$4"
STATUS2=$?

if (($STATUS1 != 0 || $STATUS2 != 0)); then
  exit 1
fi
