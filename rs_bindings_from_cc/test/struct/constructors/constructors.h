// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_

struct StructWithUserProvidedConstructors final {
  // `impl Default for StructWithUserProvidedConstructors { ... }`.
  StructWithUserProvidedConstructors();

  // TODO(lukasza): Cover copy constructor (may need [[clang::trivial_abi]]).
  // Copy constructors in elided_lifetimes.h work fine, because with lifetimes
  // the thunk's 2nd parameter is represented as `other: &'a SomeStruct`. This
  // doesn't work here (without lifetimes), when the 2nd parameter becomes
  // `other: *mut SomeStruct`.

  // `impl From<int> for StructWithUserProvidedConstructors { ... }`.
  explicit StructWithUserProvidedConstructors(int);

  int int_field;
};

// Inline-defined constructors test that thunks are properly implemented by
// `generate_rs_api_impl`.
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

struct NonTrivialStructWithConstructors {
  NonTrivialStructWithConstructors();
  explicit NonTrivialStructWithConstructors(int);

  // TODO(lukasza): Cover copy constructor (may need [[clang::trivial_abi]]).
  // This is desirable mostly for completness / parity with
  // StructWithUserProvidedConstructors.

  // Presence of a user-defined destructor makes this struct non-trivial.
  ~NonTrivialStructWithConstructors();

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
