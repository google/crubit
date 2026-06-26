// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef H_THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CONVERSION_OPERATORS_H_
#define H_THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CONVERSION_OPERATORS_H_

struct DstLocalMovable {
  int val;
};

struct DstLocalNonMovable {
  int val;
  DstLocalNonMovable(const DstLocalNonMovable&) = delete;
  DstLocalNonMovable(DstLocalNonMovable&&) = delete;
};

struct Src {
  int value;

  explicit operator const int&() const;
  explicit operator int&();

  explicit operator DstLocalMovable() const;
  explicit operator DstLocalNonMovable() const;
  explicit operator int() const;
  explicit operator double();
};

#endif  // H_THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CONVERSION_OPERATORS_H_
