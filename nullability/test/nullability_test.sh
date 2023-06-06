#!/bin/sh
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

DRIVER="$1"
shift
INCLUDES="$1"
shift

exec "$DRIVER" "$@" -- -I"$INCLUDES"

