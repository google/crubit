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
if ! $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have passed good nullable test"
  exit 1
fi
command -v grep && grep "Passed 2 test(s)" $LOG

cat >$SOURCE <<EOF
  template <class T> void nullable(T) {}

  [[clang::annotate("test")]] void badNullable(int *_Nonnull x) {
    nullable(x);
  }
EOF

if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed bad nullable() test!"
  exit 1
fi
command -v grep && grep "expression is _Nonnull, expected _Nullable" $LOG
command -v grep && grep "Passed 0 test(s)" $LOG
command -v grep && grep "Failed 2 test(s)" $LOG

cat >$SOURCE <<EOF
  template <class Expected, class Actual> void type(Actual) {}

  [[clang::annotate("test")]] void badType(int *_Nullable x) {
    type<int *_Nonnull>(x);
  }
EOF
if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed bad type() test!"
  exit 1
fi
cat $LOG
command -v grep && grep "static nullability is \[Nullable\], expected \[NonNull\]" $LOG

cat >$SOURCE <<EOF
  template <class Expected, class Actual> void type(Actual) {}
  namespace symbolic {
  template <class T> using A [[clang::annotate("symbolic_nullability:A")]] = T;
  }

  [[clang::annotate("test")]] void badSymbolicType1(symbolic::A<int *> x) {
    type<int *_Nonnull>(x);
  }
  [[clang::annotate("test")]] void badSymbolicType2(int ** _Nullable y) {
    type<symbolic::A<int *>*>(y);
  }
EOF
if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed bad symbolic type() test!"
  exit 1
fi
cat $LOG
command -v grep && grep "static nullability is \[Symbolic(.*)\], expected \[NonNull\]" $LOG
command -v grep && grep "static nullability is \[Nullable, Unspecified\], expected \[Unspecified, Symbolic(.*)\]" $LOG

cat >$SOURCE <<EOF
  [[clang::annotate("test")]] int x;
EOF
if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed with 'test' annotation attached to wrong node!"
  exit 1
fi
command -v grep && grep "TEST on VarDecl node is not supported" $LOG

cat >$SOURCE <<EOF
  void provable(bool b) {}
  void possible(bool b) {}

  [[clang::annotate("test")]] void goodProvablePossible(bool b) {
    possible(b);
    possible(!b);
    if (b)
      provable(b);
  }
EOF
if ! $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have passed good provable/possible test"
  exit 1
fi

cat >$SOURCE <<EOF
  void provable(bool b) {}

  [[clang::annotate("test")]] void badProvable(bool b) {
    provable(b);
  }
EOF

if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed bad provable() test!"
  exit 1
fi
command -v grep && grep "expression cannot be proved true" $LOG

cat >$SOURCE <<EOF
  void possible(bool b) {}

  [[clang::annotate("test")]] void badPossible(bool b) {
    if (!b)
      possible(b);
  }
EOF

if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed bad possible() test!"
  exit 1
fi
command -v grep && grep "expression is provably false" $LOG

cat >$SOURCE <<EOF
  void provable(bool b) {}
  bool f();

  [[clang::annotate("test")]] void noValueForProvable() {
    provable(f());
  }
EOF

if $DRIVER $SOURCE -- 2> $LOG; then
  echo "Should have failed no-value test!"
  exit 1
fi
command -v grep && grep "no value for boolean expression" $LOG

cat >$SOURCE <<EOF
  [[clang::annotate("test")]] void test1() {
  }
  [[clang::annotate("test")]] void test2() {
  }
EOF

if ! $DRIVER $SOURCE -- 2> $LOG; then
  echo "Error running empty test filter test"
  exit 1
fi
command -v grep && grep "test1" $LOG
command -v grep && grep "test2" $LOG

if ! TESTBRIDGE_TEST_ONLY='*test1*' $DRIVER $SOURCE -- 2> $LOG; then
  echo "Error running '*test1*' test filter test"
  exit 1
fi
command -v grep && grep "test1" $LOG
command -v grep && grep -v "test2" $LOG

if ! TESTBRIDGE_TEST_ONLY='*test1*:*test2*' $DRIVER $SOURCE -- 2> $LOG; then
  echo "Error running '*test1*:*test2*' test filter test"
  exit 1
fi
command -v grep && grep "test1" $LOG
command -v grep && grep "test2" $LOG

if ! TESTBRIDGE_TEST_ONLY='-*test2*' $DRIVER $SOURCE -- 2> $LOG; then
  echo "Error running '-*test2*' test filter test"
  exit 1
fi
command -v grep && grep "test1" $LOG
command -v grep && grep -v "test2" $LOG

if TESTBRIDGE_TEST_ONLY='[' $DRIVER $SOURCE -- 2> $LOG; then
  echo "Test filter test with invalid test syntax should fail"
  exit 1
fi
command -v grep && grep "invalid glob pattern" $LOG
