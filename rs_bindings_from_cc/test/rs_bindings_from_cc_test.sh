#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


source gbash.sh || exit
source module gbash_unit.sh

# Find input files
readonly RS_BINDINGS_FROM_CC="${RUNFILES}/rs_bindings_from_cc/rs_bindings_from_cc"

function test::cmd_line_api() {
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

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" 2>&1 | \
      grep 'please specify at least one header in --public_headers' > /dev/null" \
    "generator should show help message for --public_headers"

  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "int foo();" > "${hdr}"
  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --public_headers=\"${hdr}\" 1>&2"

  EXPECT_FILE_NOT_EMPTY "${rs_out}"
  EXPECT_FILE_NOT_EMPTY "${cc_out}"
}

function test::tool_returns_nonzero_on_invalid_input() {
  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"

  # Creating outputs so we can observe if the tool deletes them.
  touch "${rs_out}" "${cc_out}"

  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "int foo(); But this is not C++;" > "${hdr}"
  EXPECT_FAIL \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --public_headers=\"${hdr}\" 1>&2"

  CHECK_FILE_NOT_EXISTS "${rs_out}"
  CHECK_FILE_NOT_EXISTS "${cc_out}"
}

function test::public_headers() {
  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"

  local header_1="${TEST_TMPDIR}/header_1.h"
  local header_2="${TEST_TMPDIR}/header_2.h"
  echo "int function_1();" > "${header_1}"
  echo "int function_2();" > "${header_2}"
  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --public_headers=\"${header_1},${header_2}\" \
      1>&2"
  EXPECT_SUCCEED "grep function_1 \"${rs_out}\"" "function_1 was not imported"
  EXPECT_SUCCEED "grep function_2 \"${rs_out}\"" "function_2 was not imported"
}

gbash::unit::main "$@"