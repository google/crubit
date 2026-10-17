// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_INHERITED_METHODS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_INHERITED_METHODS_H_

#pragma clang lifetime_elision

#include "rs_bindings_from_cc/test/struct/inheritance/not_crubit_enabled.h"

struct Nonmovable {
  Nonmovable() = default;
  Nonmovable(Nonmovable&&) = delete;
  Nonmovable& operator=(Nonmovable&&) = delete;
};

struct Base {
  bool has_bindings() const { return true; }
  void no_bindings(Nonmovable) const {}

  // Methods using a type from a target without Crubit don't get bindings on
  // `Base` (no thunk is generated), so they must not be forwarded from
  // `Derived` either (b/565343531).
  void takes_not_crubit_enabled(NotCrubitEnabled* p) {}
  NotCrubitEnabled* returns_not_crubit_enabled() { return nullptr; }
  // `Base::overloaded` gets no bindings because it is overloaded, even though
  // only one of the overloads could be bound on its own.
  void overloaded(int i) {}
  void overloaded(int i, NotCrubitEnabled* p) {}

  // Static member functions are inherited by `Derived`, but unlike instance
  // methods they have no implicit `this` parameter. Parameters naming the
  // enclosing class must therefore keep their declared type in the bindings
  // generated for `Derived`, rather than being rewritten to `Self`.
  static void static_no_params() {}
  static void static_ptr_param(Base* a) {}
  static void static_two_ptr_params(Base* a, Base* b) {}
  static void static_ref_param(const Base& a) {}
  static void static_value_param(Base a) {}
  static Base* static_ptr_return(Base* a) { return a; }
};

struct Derived : Base {};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_INHERITED_METHODS_H_
