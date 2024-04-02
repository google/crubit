// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for recovery of nullability type information that is lost in the AST.

#include "nullability_test.h"

template <typename T>
T Instance = {};
template <typename T>
struct StaticWrapper {
  constexpr static T Value = {};
};
template <typename T>
struct Wrapper {
  T Value = {};
  T get() { return Value; }
};

TEST void testDeclRefExpr() {
  unknown(Instance<Nonnull<int *>>);              // TODO(b/332558689): nonnull
  unknown(StaticWrapper<Nonnull<int *>>::Value);  // TODO(b/332558689): nonnull
}

TEST void testMemberExpr(Wrapper<Nonnull<int *>> &W) {
  nonnull(W.Value);
  nonnull(W.get());

  struct Derived : Wrapper<char>, Wrapper<int *> {
  } D;
  unknown(D.Wrapper<Nonnull<int *>>::Value);  // TODO(b/332558689): nonnull
}

TEST void testCXXTemporaryExpr() {
  type<Wrapper<NullabilityUnknown<int *>>>(  // TODO(b/332562229): nonnull
      Wrapper<Nonnull<int *>>{});
}
