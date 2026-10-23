// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_SPACESHIP_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_SPACESHIP_H_

#include <compare>

// Defaulted `operator<=>` returning `std::strong_ordering`. The compiler also
// implicitly declares a defaulted `operator==`. Should generate
// `impl PartialEq` and `impl PartialOrd`.
struct DefaultedStrongOrdering final {
  int i;

  auto operator<=>(const DefaultedStrongOrdering&) const = default;
};

// User-defined `operator<=>` returning `std::weak_ordering`, together with a
// consistent, user-defined `operator==`. Only the last digit is compared.
struct CustomWeakOrdering final {
  int i;

  std::weak_ordering operator<=>(const CustomWeakOrdering& other) const {
    return (i % 10) <=> (other.i % 10);
  }
  bool operator==(const CustomWeakOrdering& other) const {
    return (i % 10) == (other.i % 10);
  }
};

// Defaulted `operator<=>` on a struct with a `float` member returns
// `std::partial_ordering`, which can be `unordered` (e.g. for NaN).
struct DefaultedPartialOrdering final {
  float f;

  auto operator<=>(const DefaultedPartialOrdering&) const = default;
};

// Non-member (hidden friend) `operator<=>` and `operator==`.
struct FriendStrongOrdering final {
  int i;

  friend std::strong_ordering operator<=>(const FriendStrongOrdering& lhs,
                                          const FriendStrongOrdering& rhs) {
    return lhs.i <=> rhs.i;
  }
  friend bool operator==(const FriendStrongOrdering& lhs,
                         const FriendStrongOrdering& rhs) {
    return lhs.i == rhs.i;
  }
};

// Both `operator<=>` and `operator<`. `operator<=>` should back the
// `PartialOrd` impl.
struct SpaceshipAndLessThan final {
  int i;

  auto operator<=>(const SpaceshipAndLessThan&) const = default;
  // Deliberately inconsistent with `operator<=>` so that the test can tell
  // whether this is called.
  bool operator<(const SpaceshipAndLessThan& other) const { return false; }
};

// `operator<=>` that doesn't return a comparison category type (e.g.
// `std::strong_ordering`). Shouldn't generate `impl PartialOrd`.
struct NotAnOrdering final {};
struct SpaceshipReturningNotAnOrdering final {
  int i;

  NotAnOrdering operator<=>(const SpaceshipReturningNotAnOrdering&) const {
    return {};
  }
  bool operator==(const SpaceshipReturningNotAnOrdering&) const = default;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_SPACESHIP_H_
