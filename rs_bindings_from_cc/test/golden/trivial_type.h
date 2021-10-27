// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

// Implicitly defined special member functions are trivial on a struct with
// only trivial members.
struct Trivial {
  int trivial_field;
};

// Defaulted special member functions are trivial on a struct with only trivial
// members.
struct TrivialWithDefaulted {
  TrivialWithDefaulted() = default;

  TrivialWithDefaulted(const TrivialWithDefaulted&) = default;
  TrivialWithDefaulted& operator=(const TrivialWithDefaulted&) = default;
  TrivialWithDefaulted(TrivialWithDefaulted&&) = default;
  TrivialWithDefaulted& operator=(TrivialWithDefaulted&&) = default;

  ~TrivialWithDefaulted() = default;

  int trivial_field;
};

void TakesByValue(Trivial trivial);

void TakesWithDefaultedByValue(TrivialWithDefaulted trivial);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_
