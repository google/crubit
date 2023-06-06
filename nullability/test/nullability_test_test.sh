#!/bin/sh -ex
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

SOURCE="$TEST_TMPDIR/test.cc"
LOG="$TEST_TMPDIR/test.log"
DRIVER="$1"

cat >$SOURCE <<EOF
  template <class T> void nullable(T) {}

  [[clang::annotate("test")]] void good(int *_Nullable x) {
    nullable(x);
  }
EOF
if ! $DRIVER $SOURCE -- > $LOG; then
  echo "Should have passed good test"
  exit 1
fi

cat >$SOURCE <<EOF
  template <class T> void nullable(T) {}

  [[clang::annotate("test")]] void badNullable(int *_Nonnull x) {
    nullable(x);
  }
EOF

if $DRIVER $SOURCE -- > $LOG; then
  echo "Should have failed bad nullable() test!"
  exit 1
fi
command -v grep && grep "expression is _Nonnull, expected _Nullable" $LOG

cat >$SOURCE <<EOF
  template <class Expected, class Actual> void type(Actual) {}

  [[clang::annotate("test")]] void badType(int *_Nullable x) {
    type<int *_Nonnull>(x);
  }
EOF
if $DRIVER $SOURCE -- > $LOG; then
  echo "Should have failed bad type() test!"
  exit 1
fi
command -v grep && grep "static nullability is \[_Nullable\], expected \[_Nonnull\]" $LOG

cat >$SOURCE <<EOF
  [[clang::annotate("test")]] int x;
EOF
if $DRIVER $SOURCE -- > $LOG; then
  echo "Should have failed with 'test' annotation attached to wrong node!"
  exit 1
fi
command -v grep && grep "TEST on VarDecl node is not supported" $LOG
