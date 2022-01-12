// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_ELIDED_LIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_ELIDED_LIFETIMES_H_

#pragma clang lifetime_elision

// Only trivially relocatable structs may implement Rust's Default, From<T>, or
// Clone traits. To indicate that the struct below is trivially relocatable, we
// need to use the [[clang::trivial_abi]] attribute (because otherwise the
// presence of a user-defined copy constructor might mean that the struct is
// not trivially relocatable).
struct [[clang::trivial_abi]] ElidedLifetimes {
  ElidedLifetimes();
  ElidedLifetimes(const ElidedLifetimes& other);
  explicit ElidedLifetimes(int i);
  int int_field;
};

struct [[clang::trivial_abi]] ElidedLifetimesWithInlineConstructors {
  ElidedLifetimesWithInlineConstructors() : int_field(321) {}
  ElidedLifetimesWithInlineConstructors(
      const ElidedLifetimesWithInlineConstructors& other)
      : int_field(20000 + other.int_field) {}
  explicit ElidedLifetimesWithInlineConstructors(int i) : int_field(i) {}
  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_ELIDED_LIFETIMES_H_
