// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_

struct StructWithUserProvidedConstructor {
  StructWithUserProvidedConstructor();
  // TODO(lukasza): Add a copy constructor (to be mapped to Clone?).
  // TODO(b/208946210): Add a "conversion" constructor (to be mapped to From).

  int int_field;
};

// TODO(b/200065650): Add StructWithDeletedConstructors test.
// Hint: assert_not_impl_all!(StructWithDeletedConstructors: Default);

struct StructWithPrivateConstructor {
 private:
  StructWithPrivateConstructor();

  int int_field;
};

// TODO(lukasza): Add StructWithExplicitlyDefaultedConstructor test.
// TODO(lukasza): Add StructWithImplicitlyDefaultedConstructor test (or is
//                that just testing the compiler and therefore not useful?).

// TODO(lukasza): Add NonTrivialStructWithConstructors test.

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_CONSTRUCTORS_H_
