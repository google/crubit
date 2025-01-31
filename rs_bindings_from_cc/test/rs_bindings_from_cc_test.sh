#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


source gbash.sh || exit
source module gbash_unit.sh

readonly RS_BINDINGS_FROM_CC="${RUNFILES}/rs_bindings_from_cc/rs_bindings_from_cc"
readonly DEFAULT_CLANG_FORMAT_EXE_PATH="${RUNFILES}/clang-format"
readonly DEFAULT_RUSTFMT_EXE_PATH="third_party/crosstool/rust/unstable/rustfmt_for_crubit_aspects"

function quote_escape() {
  sed 's/"/\\"/g'
}

function test::cmd_line_api() {
  EXPECT_FAIL "${RS_BINDINGS_FROM_CC}" "generator should return non-zero with no arguments"
  EXPECT_SUCCEED \
    "${RS_BINDINGS_FROM_CC} 2>&1 | grep 'please specify --target' > /dev/null" \
    "generator should show help message for --target"
  EXPECT_SUCCEED \
    "${RS_BINDINGS_FROM_CC} --target=//:target 2>&1 | grep 'please specify --rs_out' > /dev/null" \
    "generator should show help message for --rs_out"
  EXPECT_SUCCEED \
    "${RS_BINDINGS_FROM_CC} --target=//:target --rs_out=/foo 2>&1 | \
      grep 'please specify --cc_out' > /dev/null" \
    "generator should show help message for --cc_out"

  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" 2>&1 | \
      grep 'please specify --crubit_support_path_format' > /dev/null" \
    "generator should show help message for --crubit_support_path_format"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" 2>&1 \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" | \
      grep 'please specify --clang_format_exe_path' > /dev/null" \
    "generator should show help message for --clang_format_exe_path"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" 2>&1 \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" | \
      grep 'please specify --rustfmt_exe_path' > /dev/null" \
    "generator should show help message for --rustfmt_exe_path"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" 2>&1 \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" | \
      grep 'please specify --public_headers' > /dev/null" \
    "generator should show help message for --public_headers"

  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "int foo();" > "${hdr}"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${hdr}\" 2>&1 | \
      grep 'please specify --target_args' > /dev/null" \
    "generator should show help message for --target_args"

  local json
  json="$(cat <<-EOT
  [{"t": "//foo/bar:baz", "h": ["${hdr}"], "f": ["experimental", "supported"]}]
EOT
)"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${hdr}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\""

  EXPECT_FILE_NOT_EMPTY "${rs_out}"
  EXPECT_FILE_NOT_EMPTY "${cc_out}"
}

function test::do_nothing() {
  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"
  local hdr="no_such_file.h"
  local json
  json="$(cat <<-EOT
  [{"t": "//foo/bar:baz", "h": ["${hdr}"], "f": ["experimental", "supported"]}]
EOT
)"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${hdr}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\" \
      --do_nothing"

  EXPECT_FILE_NOT_EMPTY "${rs_out}"
  EXPECT_FILE_NOT_EMPTY "${cc_out}"

  EXPECT_SUCCEED "cat \"${rs_out}\" | grep '// intentionally left empty because --do_nothing was passed.'"
  EXPECT_SUCCEED "cat \"${cc_out}\" | grep '// intentionally left empty because --do_nothing was passed.'"
}

function test::tool_returns_nonzero_on_invalid_input() {
  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"
  rm -rf "$rs_out"
  rm -rf "$cc_out"

  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "int foo(); But this is not C++;" > "${hdr}"
  local json
  json="$(cat <<-EOT
  [{"t": "//foo/bar:baz", "h": ["${hdr}"], "f": ["experimental", "supported"]}]
EOT
)"

  EXPECT_FAIL \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//:target \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${hdr}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\" 2>&1"

  # No output files should be created if the C++ input was invalid.
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

  local json
  json="$(cat <<-EOT
  [{"t": "//foo/bar:baz", "h": ["${header_1}", "${header_2}"], "f": ["experimental", "supported"]}]
EOT
)"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//foo/bar:baz \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${header_1},${header_2}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\" 2>&1"

  EXPECT_SUCCEED "grep function_1 \"${rs_out}\"" "function_1 was not imported"
  EXPECT_SUCCEED "grep function_2 \"${rs_out}\"" "function_2 was not imported"
}

function test::rustfmt_config_path() {
  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"

  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "float MyFunction(float arg1, float arg2);" > "${hdr}"

  local json
  json="$(cat <<-EOT
  [{"t": "//foo/bar:baz", "h": ["${hdr}"], "f": ["experimental", "supported"]}]
EOT
)"

  #########################################################
  # Testing the default `rustfmt` config.
  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//foo/bar:baz \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${hdr}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\""

  EXPECT_FILE_NOT_EMPTY "${rs_out}"

  # Expecting:
  #     pub fn MyFunction(arg1: f32, arg2: f32) -> f32
  EXPECT_SUCCEED \
    "grep \"MyFunction.*arg1:.*f32,.*arg2:.*f32.*)\" \"${rs_out}\"" \
    "Verify function args are on single line when using default rustfmt config (1)"
  EXPECT_FAIL \
    "grep \"^[^a-z]*arg1:[^a-z]*f32,[^a-z]*\\\$\" \"${rs_out}\"" \
    "Verify function args are on single line when using default rustfmt config (2)"

  #########################################################
  # Testing a custom `rustfmt` config.
  local rustfmt_config_path="${TEST_TMPDIR}/rustfmt.toml"
  cat >"${rustfmt_config_path}" <<EOF
    edition = "2021"
    version = "Two"
    fn_args_layout="Vertical"
EOF
  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//foo/bar:baz\
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --rustfmt_config_path=\"${rustfmt_config_path}\" \
      --public_headers=\"${hdr}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\""

  EXPECT_FILE_NOT_EMPTY "${rs_out}"

  # Expecting:
  #     pub fn MyFunction(
  #         arg1: f32,
  #         arg2: f32,
  #     ) -> f32
  EXPECT_FAIL \
    "grep \"MyFunction.*arg1:.*f32,.*arg2:.*f32.*)\" \"${rs_out}\"" \
    "Verify function args are *not* on single line when using custom rustfmt config (1)"
  EXPECT_SUCCEED \
    "grep \"^[^a-z]*arg1:[^a-z]*f32,[^a-z]*\\\$\" \"${rs_out}\"" \
    "Verify function args are *not* on single line when using custom rustfmt config (2)"
}

function test::crubit_support_path_format() {
  local rs_out="${TEST_TMPDIR}/rs_api.rs"
  local cc_out="${TEST_TMPDIR}/rs_api_impl.cc"

  local hdr="${TEST_TMPDIR}/hello_world.h"
  echo "int MyFunction();" > "${hdr}"

  local json
  json="$(cat <<-EOT
  [{"t": "//foo/bar:baz", "h": ["${hdr}"], "f": ["experimental", "supported"]}]
EOT
)"

  EXPECT_SUCCEED \
    "\"${RS_BINDINGS_FROM_CC}\" \
      --target=//foo/bar:baz \
      --rs_out=\"${rs_out}\" \
      --cc_out=\"${cc_out}\" \
      --crubit_support_path_format=\"<test/specific/crubit/support/path/{header}>\" \
      --clang_format_exe_path=\"${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
      --rustfmt_exe_path=\"${DEFAULT_RUSTFMT_EXE_PATH}\" \
      --public_headers=\"${hdr}\" \
      --target_args=\"$(echo "${json}" | quote_escape)\""

  EXPECT_FILE_NOT_EMPTY "${cc_out}"
  EXPECT_SUCCEED \
    "grep \"#include <test/specific/crubit/support/path/.*>\" \"${cc_out}\"" \
    "Verify #include paths are based on the argument of --crubit_support_path_format"
}

gbash::unit::main "$@"
