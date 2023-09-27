#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# This file provides test coverage for aspects of `cc_bindings_from_rs.rs` that
# are not covered by the native Rust tests (from the same source file -
# `cc_bindings_from_rs.rs`).

source gbash.sh || exit
source module gbash_unit.sh


readonly STDERR_PATH="${TEST_TMPDIR}/stderr.txt"
readonly STDOUT_PATH="${TEST_TMPDIR}/stdout.txt"
readonly H_OUT_PATH="${TEST_TMPDIR}/cc_api.h"
readonly RS_OUT_PATH="${TEST_TMPDIR}/cc_api_impl.rs"
function delete_all_test_outputs() {
  rm -rf "$STDERR_PATH" "$STDOUT_PATH" "$H_OUT_PATH" "$RS_OUT_PATH"
}

readonly CC_BINDINGS_FROM_RS_PATH="${RUNFILES}/cc_bindings_from_rs/cc_bindings_from_rs"
readonly SYSROOT_PATH="${RUNFILES}/google3/nowhere/llvm/rust/main_sysroot"
readonly RUSTFMT_PATH="nowhere/llvm/rust/main_sysroot/bin/rustfmt"
readonly DEFAULT_CLANG_FORMAT_EXE_PATH="${RUNFILES}/google3/third_party/crosstool/google3_users/clang-format"

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
    "\"$CC_BINDINGS_FROM_RS_PATH\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        \"--h-out=${H_OUT_PATH}\" \
        \"--rs-out=${RS_OUT_PATH}\" \
        \"--crubit-support-path-format=<crubit/support/{header}>\" \
        \"--clang-format-exe-path=${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
        \"--rustfmt-exe-path=$RUSTFMT_PATH\" \
        -- \
        \"$RS_INPUT_PATH\" \
        --crate-type=lib \
        --sysroot=$SYSROOT_PATH \
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
    "\"$CC_BINDINGS_FROM_RS_PATH\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        --help" \
    "--help should print the error message and return with 0 exit code"

  EXPECT_STR_EMPTY "$(cat $STDERR_PATH)"

  EXPECT_SUCCEED \
    "grep 'Generates C++ bindings for a Rust crate' \"$STDOUT_PATH\" >/dev/null" \
    "The help message should contain the introduction"
  EXPECT_SUCCEED \
    "grep 'Output path for C++ header file with bindings' \
            \"$STDOUT_PATH\" >/dev/null" \
    "The help message should contain flag-specific snippets"
}

# This tests that `main` correctly special-cases errors received from the `clap`
# crate.  In particular, calling `clap::Error::exit` should return zero exit
# code when an unrecognized Crubit flag is used.
function test::unrecognized_crubit_flag() {
  delete_all_test_outputs
  EXPECT_FAIL \
    "\"$CC_BINDINGS_FROM_RS_PATH\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        --no-such-crubit-flag" \
    "Unrecognized cmdline flag should result in non-0 exit code"

  EXPECT_STR_EMPTY "$(cat $STDOUT_PATH)"

  EXPECT_SUCCEED \
    "grep 'unexpected argument .*--no-such-crubit-flag.*' \"$STDERR_PATH\" >/dev/null" \
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
    "\"$CC_BINDINGS_FROM_RS_PATH\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        --h-out=../.. \
        --rs-out=blah \
        \"--crubit-support-path-format=<crubit/support/{header}>\" \
        \"--clang-format-exe-path=${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
        \"--rustfmt-exe-path=$RUSTFMT_PATH\" \
        -- \
        \"$RS_INPUT_PATH\" \
        --crate-type=lib \
        --sysroot=$SYSROOT_PATH \
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

# This test verifies that running `cc_bindings_from_rs` will not emit the
# following warning: use of deprecated function `deprecated_function` (note:
# `#[warn(deprecated)]` on by default).  The motivation for silencing the
# warnings is the desire to avoid reporting warnings twice (once via `rustc` +
# once via `cc_bindings_from_rs`).
function test::rustc_warnings_are_silenced() {
  local RS_INPUT_PATH="${TEST_TMPDIR}/crate_name.rs"
  echo >"$RS_INPUT_PATH" "
      #[deprecated]
      fn deprecated_function() {
          unimplemented!()
      }

      pub fn public_function() {
          deprecated_function()
      }
  "

  delete_all_test_outputs
  EXPECT_SUCCEED \
    "\"$CC_BINDINGS_FROM_RS_PATH\" >\"$STDOUT_PATH\" 2>\"$STDERR_PATH\" \
        \"--h-out=${H_OUT_PATH}\" \
        \"--rs-out=${RS_OUT_PATH}\" \
        \"--crubit-support-path-format=<crubit/support/{header}>\" \
        \"--clang-format-exe-path=${DEFAULT_CLANG_FORMAT_EXE_PATH}\" \
        \"--rustfmt-exe-path=$RUSTFMT_PATH\" \
        -- \
        \"$RS_INPUT_PATH\" \
        --crate-type=lib \
        --sysroot=$SYSROOT_PATH \
        --codegen=panic=abort" \
    "Expecting that this invocation of cc_bindings_from_rs will succeed"

  EXPECT_STR_EMPTY "$(cat $STDOUT_PATH)"
  EXPECT_STR_EMPTY "$(cat $STDERR_PATH)"
}

gbash::unit::main "$@"
