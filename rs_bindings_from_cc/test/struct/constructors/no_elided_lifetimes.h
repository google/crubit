// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_NO_ELIDED_LIFETIMES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_NO_ELIDED_LIFETIMES_H_

// `[[clang::trivial_abi]]` is used so that `is_trivial_abi` doesn't get in the
// way of generating binding for constructors - we want to test that
// `Clone::clone`, `Default::default`, `From::from` are skipped not because of
// `!is_trivial_abi`, but because of a missing lifetime annotations for `__this`
// (and for the parameter of the copy constructor).
struct [[clang::trivial_abi]] StructWithConstructorsWithoutLifetimes final {
  StructWithConstructorsWithoutLifetimes();

  // No `impl Clone for StructWithUserProvidedConstructors` is expected, because
  // without lifetimes (e.g. without `#pragma clang lifetime_elision`) we should
  // not translate the `other` parameter into `&self` in `Clone::clone()`
  // (without lifetimes `other` should be `*const StructWithUser...`).
  StructWithConstructorsWithoutLifetimes(
      const StructWithConstructorsWithoutLifetimes&);

  // NOLINTNEXTLINE(google-explicit-constructor)
  StructWithConstructorsWithoutLifetimes(int);

  int int_field;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_NO_ELIDED_LIFETIMES_H_
