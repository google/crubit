// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATOR_MIXINS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATOR_MIXINS_H_

// CRTP mixins that define operators for the derived class `T` as hidden
// friends. Crubit is not enabled for this library.

namespace mixins {

template <typename T>
struct EqualityMixin {
  friend bool operator==(const T& lhs, const T& rhs) {
    return lhs.value == rhs.value;
  }
  friend bool operator!=(const T& lhs, const T& rhs) { return !(lhs == rhs); }
};

template <typename T>
struct OrderingMixin {
  friend bool operator<(const T& lhs, const T& rhs) {
    return lhs.value < rhs.value;
  }
};

// Inherits the operators of other mixins.
template <typename T>
struct ComparisonMixin : EqualityMixin<T>, OrderingMixin<T> {};

// Defines an operator whose first parameter is not of type `T`.
template <typename T>
struct IntEqualityMixin {
  friend bool operator==(int lhs, const T& rhs) { return lhs == rhs.value; }
};

}  // namespace mixins

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATOR_MIXINS_H_
