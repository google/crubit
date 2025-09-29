#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

set -euo pipefail

if [ $# -ne 0 ]; then
  TESTS_TO_RUN=$@
else
  TESTS_TO_RUN=(//...)
fi

bazel test \
  --test_tag_filters=crubit_golden_test,-manual \
  --build_tag_filters=crubit_golden_test,-manual \
  --config=llvm-unstable \
  --test_strategy=local \
  --test_env=WRITE_GOLDENS=1 \
  --cache_test_results=no \
  -k \
  $TESTS_TO_RUN
