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

// TODO(lukasza): StructWithInlinedConstructors (to force thunk generation).

struct StructWithDeletedConstructor final {
  StructWithDeletedConstructor() = delete;

  int int_field;
};

struct StructWithPrivateConstructor final {
 private:
  StructWithPrivateConstructor();

  int int_field;
};

struct StructWithExplicitlyDefaultedConstructor final {
  StructWithExplicitlyDefaultedConstructor() = default;

  int field_with_explicit_initializer = 123;
  int field_with_no_initializer;
};

// TODO(lukasza): Add StructWithImplicitlyDefaultedConstructor test (or is
//                that just testing the compiler and therefore not useful?).

struct NonTrivialStructWithConstructors {
  NonTrivialStructWithConstructors();

  // Presence of a user-defined destructor makes this struct non-trivial.
  ~NonTrivialStructWithConstructors();

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
