// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for various forms of initialization.

#include "nullability_test.h"

TEST void valueInitializedPointerIsNull() {
  using Pointer = int *;
  provable(Pointer() == nullptr);
}

template <class>
struct Movable {
  Movable() = default;
  Movable(Movable &&) = default;
};
template <class X>
Movable(X) -> Movable<X>;
template <class>
struct Copyable {
  Copyable() = default;
  Copyable(const Copyable &) = default;
};
template <class X>
Copyable(X) -> Copyable<X>;

namespace std {
template <typename T>
T &&move(T &X) {
  return static_cast<T &&>(X);
}
template <typename T>
T &&forward(T &&X) {
  return static_cast<T &&>(X);
}
}  // namespace std

TEST void testClassCopyMove(Copyable<Nullable<int *>> B) {
  // We want to test that type nullability is preserved when passing
  // args by value, when that passing introduces an implicit copy/move.
  // Passing directly into type<>() won't do this, as it uses universal
  // references for its arguments.
  //
  // Since writing the template args explicitly is arguably a nullability cast,
  // we use CTAD as an approximation of the implicit copy/move.
  type<Copyable<Nullable<int *>>>(Copyable(B));

  Movable<Nullable<int *>> makeMovable(/*suppress -Wvexing-parse*/ int);
  type<Movable<Nullable<int *>>>(makeMovable(0));
}

TEST void testMoveForward(Copyable<Nullable<int *>> A,
                          Movable<Nullable<int *>> B) {
  type<Copyable<Nullable<int *>>>(std::forward<Copyable<int *> &>(A));
  type<Movable<Nullable<int *>>>(std::move(B));
}
