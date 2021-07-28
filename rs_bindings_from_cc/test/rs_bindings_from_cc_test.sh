#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


source gbash.sh || exit
source module gbash_unit.sh

# Find input files
readonly RS_BINDINGS_FROM_CC="${RUNFILES}/rs_bindings_from_cc/rs_bindings_from_cc"

function test::rs_bindings_from_cc_cmd_line_api() {
  EXPECT_FAIL "${RS_BINDINGS_FROM_CC}" "generator should return non-zero with no arguments"
  EXPECT_SUCCEED \
    "${RS_BINDINGS_FROM_CC} 2>&1 | grep 'please specify --rs_out' > /dev/null" \
    "generator should show help message for --rs_out"
  EXPECT_SUCCEED \
    "${RS_BINDINGS_FROM_CC} --rs_out=/foo 2>&1 | \
      grep 'please specify --cc_out' > /dev/null" \
    "generator should show help message for --cc_out"

  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"
  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "int foo();" > "${hdr}"
  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --use_tool_args_for_compile \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      \"${hdr}\""

  EXPECT_FILE_NOT_EMPTY "${rs_out}"
  EXPECT_FILE_NOT_EMPTY "${cc_out}"
}

gbash::unit::main "$@"