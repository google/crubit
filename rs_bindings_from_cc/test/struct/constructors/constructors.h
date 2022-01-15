// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_

// `[[clang::trivial_abi]]` is used so that `is_trivial_abi` doesn't get
// in the way of generating a binding for `Clone::clone` - we want to test
// that `Clone::clone` is skipped not because of `!is_trivial_abi`, but because
// of a missing lifetime annotation for the parameter of the copy constructor.
// TODO(b/214244223): No bindings should be generated for any of the
// constructors here (because there are no lifetime annotations).
// (Currently only the copy constructor is skipped.)
struct [[clang::trivial_abi]] StructWithUserProvidedConstructors final {
  // `impl Default for StructWithUserProvidedConstructors { ... }`.
  StructWithUserProvidedConstructors();

  // No `impl Copy for StructWithUserProvidedConstructors` is expected, because
  // without lifetimes (e.g. without `#pragma clang lifetime_elision`) we should
  // not translate the `other` parameter into `&self` in `Clone::clone()`
  // (without lifetimes `other` should be `*const StructWithUser...`).
  StructWithUserProvidedConstructors(const StructWithUserProvidedConstructors&);

  // `impl From<int> for StructWithUserProvidedConstructors { ... }`.
  explicit StructWithUserProvidedConstructors(int);

  int int_field;
};

// Inline-defined constructors test that thunks are properly implemented by
// `generate_rs_api_impl`.
// TODO(b/214244223): Move this and other test scenarios below into
// `elided_lifetimes.h`.  (Maybe renaming these files along the way?)
struct StructWithInlineConstructors final {
  StructWithInlineConstructors() : int_field(123) {}

  // TODO(lukasza): Cover copy constructor (may need [[clang::trivial_abi]]).
  // This is desirable mostly for completness / parity with
  // StructWithUserProvidedConstructors.

  explicit StructWithInlineConstructors(int i) : int_field(i) {}
  int int_field;
};

struct StructWithDeletedConstructors final {
  StructWithDeletedConstructors() = delete;
  StructWithDeletedConstructors(const StructWithDeletedConstructors&) = delete;
  explicit StructWithDeletedConstructors(int) = delete;

  int int_field;
};

struct StructWithPrivateConstructors final {
 private:
  StructWithPrivateConstructors();
  StructWithPrivateConstructors(const StructWithPrivateConstructors&);
  explicit StructWithPrivateConstructors(int);

  int int_field;
};

struct StructWithExplicitlyDefaultedConstructors final {
  StructWithExplicitlyDefaultedConstructors() = default;
  StructWithExplicitlyDefaultedConstructors(
      const StructWithExplicitlyDefaultedConstructors&) = default;

  int field_with_explicit_initializer = 123;
  int field_with_no_initializer;
};

// TODO(lukasza): Add StructWithImplicitlyDefaultedConstructor test (or is
//                that just testing the compiler and therefore not useful?).

struct NonTrivialStructWithConstructors final {
  NonTrivialStructWithConstructors();
  explicit NonTrivialStructWithConstructors(int);

  // TODO(lukasza): Cover copy constructor (may need [[clang::trivial_abi]]).
  // This is desirable mostly for completness / parity with
  // StructWithUserProvidedConstructors. See <internal link> for
  // more details (e.g. what should be the return type of `Default::default()`)

  // Presence of a user-defined destructor makes this struct non-trivial.
  ~NonTrivialStructWithConstructors();

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
