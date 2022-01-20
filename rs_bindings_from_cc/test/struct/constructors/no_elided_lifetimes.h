// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_NO_ELIDED_LIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_NO_ELIDED_LIFETIMES_H_

// `[[clang::trivial_abi]]` is used so that `is_trivial_abi` doesn't get
// in the way of generating a binding for `Clone::clone` - we want to test
// that `Clone::clone` is skipped not because of `!is_trivial_abi`, but because
// of a missing lifetime annotation for the parameter of the copy constructor.
// TODO(b/214244223): No bindings should be generated for any of the
// constructors here (because there are no lifetime annotations).
// (Currently only the copy constructor is skipped.)
struct [[clang::trivial_abi]] StructWithConstructorsWithoutLifetimes final {
  // `impl Default for StructWithUserProvidedConstructors { ... }`.
  StructWithConstructorsWithoutLifetimes();

  // No `impl Clone for StructWithUserProvidedConstructors` is expected, because
  // without lifetimes (e.g. without `#pragma clang lifetime_elision`) we should
  // not translate the `other` parameter into `&self` in `Clone::clone()`
  // (without lifetimes `other` should be `*const StructWithUser...`).
  StructWithConstructorsWithoutLifetimes(
      const StructWithConstructorsWithoutLifetimes&);

  // `impl From<int> for StructWithUserProvidedConstructors { ... }`.
  explicit StructWithConstructorsWithoutLifetimes(int);

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_NO_ELIDED_LIFETIMES_H_
