// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITED_FRIEND_OPERATORS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITED_FRIEND_OPERATORS_H_

#include "rs_bindings_from_cc/test/golden/operator_mixins.h"

// Hidden friends of base classes are found by argument-dependent lookup, so
// with the `inherited_friend_operators` feature, the operators defined by
// `EqualityMixin` get bindings, even though they are declared in another
// target.
struct Equatable final : mixins::EqualityMixin<Equatable> {
  int value;
};

// Operators are inherited from indirect bases, too.
struct Comparable final : mixins::ComparisonMixin<Comparable> {
  int value;
};

namespace ns {
struct NamespacedEquatable final : mixins::EqualityMixin<NamespacedEquatable> {
  int value;
};
}  // namespace ns

struct Outer final {
  struct NestedEquatable final : mixins::EqualityMixin<NestedEquatable> {
    int value;
  };
};

// Types that aren't `Unpin` can inherit operators as well.
struct NonTrivialEquatable final : mixins::EqualityMixin<NonTrivialEquatable> {
  ~NonTrivialEquatable() {}  // NOLINT(modernize-use-equals-default)
  int value;
};

// The operators of `EqualityMixin<Equatable>` compare `Equatable`s, not
// `NotEquatable`s, so they don't get bindings here.
struct NotEquatable final : mixins::EqualityMixin<Equatable> {
  int value;
};

// Inherited operators whose first parameter is not the derived class don't get
// bindings.
struct IntEquatable final : mixins::IntEqualityMixin<IntEquatable> {
  int value;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITED_FRIEND_OPERATORS_H_
