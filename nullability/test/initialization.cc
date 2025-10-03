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

TEST void testClassCopyMove(Copyable<int *_Nullable> B) {
  // We want to test that type nullability is preserved when passing
  // args by value, when that passing introduces an implicit copy/move.
  // Passing directly into type<>() won't do this, as it uses universal
  // references for its arguments.
  //
  // Since writing the template args explicitly is arguably a nullability cast,
  // we use CTAD as an approximation of the implicit copy/move.
  type<Copyable<int *_Nullable>>(Copyable(B));

  Movable<int *_Nullable> makeMovable(/*suppress -Wvexing-parse*/ int);
  type<Movable<int *_Nullable>>(makeMovable(0));
}

TEST void testMoveForward(Copyable<int *_Nullable> A,
                          Movable<int *_Nullable> B) {
  type<Copyable<int* _Nullable>>(std::forward<Copyable<int* _Nullable>>(A));
  type<Movable<int *_Nullable>>(std::move(B));
}
