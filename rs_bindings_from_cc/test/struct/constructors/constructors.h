// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_

#pragma clang lifetime_elision

// `[[clang::trivial_abi]]` is used so that `is_trivial_abi` doesn't prevent
// generating bindings for constructors, even though the presence of a
// user-defined copy constructor technically means that the struct below
// is non-trivial.
struct [[clang::trivial_abi]] StructWithUserProvidedConstructors final {
  // `impl Default for StructWithUserProvidedConstructors { ... }`.
  StructWithUserProvidedConstructors();

  // `impl Clone for StructWithUserProvidedConstructors { ... }`.
  StructWithUserProvidedConstructors(const StructWithUserProvidedConstructors&);

  int int_field;
};

struct StructWithExplicitConversionConstructor final {
  // Testing `impl From<int> for ...` when the constructor is `explicit`.
  explicit StructWithExplicitConversionConstructor(int i) : int_field(i) {}

  int int_field;
};

struct StructWithImplicitConversionConstructor final {
  // Testing `impl From<int> for ...` when the constructor is *not* `explicit`.
  // NOLINTNEXTLINE(google-explicit-constructor)
  StructWithImplicitConversionConstructor(int i) : int_field(i) {}

  int int_field;
};

struct OtherSimpleStruct {
  int int_field;
};

struct StructWithImplicitConversionFromReference final {
  // Testing `impl<'b> From<&'b OtherSimpleStruct> for ...`.
  // NOLINTNEXTLINE(google-explicit-constructor)
  StructWithImplicitConversionFromReference(const OtherSimpleStruct& other)
      : int_field(other.int_field) {}

  int int_field;
};

// Inline-defined constructors test that thunks are properly implemented by
// `generate_rs_api_impl`.
struct [[clang::trivial_abi]] StructWithInlineConstructors final {
  StructWithInlineConstructors() : int_field(123) {}
  StructWithInlineConstructors(const StructWithInlineConstructors& other)
      : int_field(20000 + other.int_field) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  StructWithInlineConstructors(int i) : int_field(i) {}
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

  // Presence of a user-defined destructor makes this struct non-trivial.
  ~NonTrivialStructWithConstructors();

  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
