// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_

struct StructWithUserProvidedConstructors final {
  // `impl Default for StructWithUserProvidedConstructors { ... }`.
  StructWithUserProvidedConstructors();

  // TODO(lukasza): Add a copy constructor (to be mapped to Clone?)

  // `impl From<int> for StructWithUserProvidedConstructors { ... }`.
  explicit StructWithUserProvidedConstructors(int);

  int int_field;
};

// Inline-defined constructors test that thunks are properly implemented by
// `generate_rs_api_impl`.
struct StructWithInlineConstructors final {
  StructWithInlineConstructors() : int_field(123) {}
  // TODO(lukasza): Add a copy constructor (to be mapped to Clone?)
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

  // TODO(lukasza): Add a copy constructor (to be mapped to Clone?)

  // Presence of a user-defined destructor makes this struct non-trivial.
  ~NonTrivialStructWithConstructors();

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
