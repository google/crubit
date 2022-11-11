#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# This file provides test coverage for aspects of `cc_bindings_from_rs.rs` that
# are not covered by the native Rust tests (from the same source file -
# `cc_bindings_from_rs.rs`).

source gbash.sh || exit
source module gbash_unit.sh

readonly CC_BINDINGS_FROM_RS="${RUNFILES}/cc_bindings_from_rs/cc_bindings_from_rs_legacy_toolchain_runner.sar"

readonly STDERR_PATH="${TEST_TMPDIR}/stderr.txt"
readonly STDOUT_PATH="${TEST_TMPDIR}/stdout.txt"
readonly H_OUT_PATH="${TEST_TMPDIR}/cc_api.h"
readonly RS_OUT_PATH="${TEST_TMPDIR}/cc_api_impl.rs"
function delete_all_test_outputs() {
  rm -rf "$STDERR_PATH" "$STDOUT_PATH" "$H_OUT_PATH" "$RS_OUT_PATH"
}

readonly DEFAULT_RUSTFMT_EXE_PATH="third_party/unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt"

# This tests a simple happy, errors-free code path.
function test::happy_path() {
  local RS_INPUT_PATH="${TEST_TMPDIR}/crate_name.rs"
  echo >"$RS_INPUT_PATH" "
      #[no_mangle]
      pub extern \"C\" fn public_function() {
          private_function()
      }

      fn private_function() {}
  "

  delete_all_test_outputs
  EXPECT_SUCCEED \
    "\"${CC_BINDINGS_FROM_RS}\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        \"--h-out=${H_OUT_PATH}\" \
        \"--rs-out=${RS_OUT_PATH}\" \
        \"--rustfmt-exe-path=${DEFAULT_RUSTFMT_EXE_PATH}\" \
        -- \
        \"$RS_INPUT_PATH\" \
        --crate-type=lib \
        --codegen=panic=abort" \
    "Expecting that this invocation of cc_bindings_from_rs will succeed"

  EXPECT_STR_EMPTY "$(cat $STDOUT_PATH)"
  EXPECT_STR_EMPTY "$(cat $STDERR_PATH)"

  EXPECT_FILE_NOT_EMPTY "${H_OUT_PATH}"
  EXPECT_SUCCEED \
    "grep 'Automatically @generated C++ bindings for the following Rust crate:' \
        \"$H_OUT_PATH\" >/dev/null" \
    "The emitted .h file should contain a header comment"
  EXPECT_SUCCEED \
    "grep 'extern \"C\" void public_function();' \
        \"$H_OUT_PATH\" >/dev/null" \
    "The emitted .h file should contain C++ bindings"

  EXPECT_FILE_NOT_EMPTY "${RS_OUT_PATH}"
  EXPECT_SUCCEED \
    "grep 'Automatically @generated C++ bindings for the following Rust crate:' \
        \"$H_OUT_PATH\" >/dev/null" \
    "The emitted .h file should contain a header comment"
  # TODO(b/254097223): Cover the contents of the generated `.rs` file once they
  # actually contain Rust thunks.
}

# This tests that `main` special-cases errors received from the `clap` crate.
# In particular, calling `clap::Error::exit` should return zero exit code
# when `--help` flag is used.
function test::crubit_help() {
  delete_all_test_outputs
  EXPECT_SUCCEED \
    "\"${CC_BINDINGS_FROM_RS}\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        --help" \
    "--help should print the error message and return with 0 exit code"

  EXPECT_STR_EMPTY "$(cat $STDERR_PATH)"

  EXPECT_SUCCEED \
    "grep 'Generates C++ bindings for a Rust crate' \"$STDOUT_PATH\" >/dev/null" \
    "The help message should contain the introduction"
  EXPECT_SUCCEED \
    "grep '\\--h-out <FILE>.*Output path for C++ header file with bindings' \
            \"$STDOUT_PATH\" >/dev/null" \
    "The help message should contain flag-specific snippets"
}

# This tests that `main` correctly special-cases errors received from the `clap`
# crate.  In particular, calling `clap::Error::exit` should return zero exit
# code when an unrecognized Crubit flag is used.
function test::unrecognized_crubit_flag() {
  delete_all_test_outputs
  EXPECT_FAIL \
    "\"${CC_BINDINGS_FROM_RS}\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        --no-such-crubit-flag" \
    "Unrecognized cmdline flag should result in non-0 exit code"

  EXPECT_STR_EMPTY "$(cat $STDOUT_PATH)"

  EXPECT_SUCCEED \
    "grep '\\--no-such-crubit-flag.*wasn.t expected' \"$STDERR_PATH\" >/dev/null" \
    "The error message should complain about the unrecognized flag"
}

# This tests that `main` correctly propagates a non-`clap`-related error.
function test::invalid_h_out() {
  local RS_INPUT_PATH="${TEST_TMPDIR}/crate_name.rs"
  echo >"$RS_INPUT_PATH" "
      pub fn public_function() {}
  "

  # Test what happens when the `--h-out` argument below is invalid.  This
  # particular error condition results in 1) non-`clap`-error and 2) in an error
  # chain containing Crubit-level and std::fs-level errors..
  delete_all_test_outputs
  EXPECT_FAIL \
    "\"${CC_BINDINGS_FROM_RS}\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        --h-out=../.. \
        --rs-out=blah \
        \"--rustfmt-exe-path=${DEFAULT_RUSTFMT_EXE_PATH}\" \
        -- \
        \"$RS_INPUT_PATH\" \
        --crate-type=lib \
        --codegen=panic=abort" \
    "Invalid --h-out path should result in non-0 exit code"

  EXPECT_STR_EMPTY "$(cat $STDOUT_PATH)"

  EXPECT_SUCCEED \
    "grep 'Error when writing to ../..' \
        \"$STDERR_PATH\" >/dev/null" \
    "Crubit-generated part of the error message should be present"
  EXPECT_SUCCEED \
    "grep 'Is a directory (os error 21)' \
        \"$STDERR_PATH\" >/dev/null" \
    "std::fs-generated part of the error message should be present"
}

gbash::unit::main "$@"
