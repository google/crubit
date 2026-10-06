// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATOR_MIXINS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATOR_MIXINS_H_

// CRTP mixins that define comparison operators for the derived class `T` as
// hidden friends. Crubit is not enabled for this library.
//
// The operators only compare the `key` field of `T`.

namespace mixins {

template <typename T>
struct EqualityMixin {
  friend bool operator==(const T& lhs, const T& rhs) {
    return lhs.key == rhs.key;
  }
  friend bool operator!=(const T& lhs, const T& rhs) { return !(lhs == rhs); }
};

template <typename T>
struct OrderingMixin {
  friend bool operator<(const T& lhs, const T& rhs) {
    return lhs.key < rhs.key;
  }
};

template <typename T>
struct ComparisonMixin : EqualityMixin<T>, OrderingMixin<T> {};

}  // namespace mixins

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATOR_MIXINS_H_
