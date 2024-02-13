#!/bin/bash -ex
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

SOURCE="$TEST_TMPDIR/test.cc"
LOG="$TEST_TMPDIR/test.log"
DRIVER="$1"

# Runs the `nullability_test` driver, and asserts the result. Args:
# - expected outcome: 'pass' or 'fail'
# - name of the test
# - code
# - grep patterns that should appear in the log. (0 or more of these)
#   may be preceded with '!' if they *shouldn't* appear
run_test() {
  WANT="$1"
  NAME="$2"
  echo "$3" > $SOURCE
  shift 3
  echo "***** nullability_test_test: $NAME *****"

  $DRIVER $SOURCE -- 2> $LOG && GOT=pass || GOT=fail
  cat $LOG

  if [[ "$WANT" != "$GOT" ]]; then
    echo "Should $WANT: $NAME"
    exit 1
  fi
  if command -v grep; then
    for EXPECT in "$@"; do
      if [[ "$EXPECT" == '!'* ]]; then
        grep -v "${EXPECT:1}" $LOG ||{ echo "$NAME: unwanted $EXPECT"; exit 1; }
      else
        grep "$EXPECT" $LOG ||{ echo "$NAME: expected $EXPECT"; exit 1; }
      fi
    done
  fi
}

run_test pass "good nullable()" '
  template <class T> void nullable(T) {}

  [[clang::annotate("test")]] void good(int *_Nullable x) {
    nullable(x);
  }
' \
"Passed 2 test(s)"

run_test fail "bad nullable()" '
  template <class T> void nullable(T) {}

  [[clang::annotate("test")]] void badNullable(int *_Nonnull x) {
    nullable(x);
  }
' \
"expression is _Nonnull, expected _Nullable" \
"Passed 0 test(s)" \
"Failed 2 test(s)"

run_test fail "bad type()" '
  template <class Expected, class Actual> void type(Actual) {}

  [[clang::annotate("test")]] void badType(int *_Nullable x) {
    type<int *_Nonnull>(x);
  }
' \
"static nullability is \[Nullable\], expected \[NonNull\]"

run_test fail "bad symbolic type()" '
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
' \
"static nullability is \[Symbolic(.*)\], expected \[NonNull\]" \
"static nullability is \[Nullable, Unspecified\], expected \[Unspecified, Symbolic(.*)\]"

run_test fail "test annotation on wrong node" '
  [[clang::annotate("test")]] int x;
' \
"TEST on VarDecl node is not supported"

run_test pass "good provable/possible" '
  void provable(bool b) {}
  void possible(bool b) {}

  [[clang::annotate("test")]] void goodProvablePossible(bool b) {
    possible(b);
    possible(!b);
    if (b)
      provable(b);
  }
' \

run_test fail "bad provable" '
  void provable(bool b) {}

  [[clang::annotate("test")]] void badProvable(bool b) {
    provable(b);
  }
' \
"expression cannot be proved true"

run_test fail "bad possible" '
  void possible(bool b) {}

  [[clang::annotate("test")]] void badPossible(bool b) {
    if (!b)
      possible(b);
  }
' \
"expression is provably false"

run_test fail "no-value" '
  void provable(bool b) {}
  bool f();

  [[clang::annotate("test")]] void noValueForProvable() {
    provable(f());
  }
' \
 "no value for boolean expression"

run_test fail "constructor" '
  template <class T> void nonnull(T) {}

  struct S {
    [[clang::annotate("test")]] S() { nonnull((int*)nullptr); }
  };
' \
"expression is _Nullable"

# Tests for test-filtering support
run_filter_test() {
  TEST_FILTER="$1"
  WANT="$2"
  shift 2
  TEST_NAME="filter: $TEST_FILTER"

  export TESTBRIDGE_TEST_ONLY
  OLD_FILTER="$TESTBRIDGE_TEST_ONLY"
  TESTBRIDGE_TEST_ONLY="$TEST_FILTER"
  run_test "$WANT" "filter: $TEST_FILTER" "$@"
  TESTBRIDGE_TEST_ONLY="$OLD_FILTER"
}
CODE='
  [[clang::annotate("test")]] void test1() {}
  [[clang::annotate("test")]] void test2() {}
'
run_filter_test '' pass "$CODE" "test1" "test2"
run_filter_test '*test1*' pass "$CODE" "test1" "!test2"
run_filter_test '*test1*:*test2*' pass "$CODE" "test1" "test2"
run_filter_test '-*test2*' pass "$CODE" "test1" "!test2"
run_filter_test '[' fail "$CODE" "invalid glob pattern"
