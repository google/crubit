// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_INFERRED_LIFETIMES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_INFERRED_LIFETIMES_H_

#pragma clang lifetime_elision

// Not a template, so that it isn't visible to the bindings generator.
// We're just here to save typing.
#define TEST(Name, T)                            \
  struct Name {                                  \
    T field;                                     \
    static T Function(T param) { return param; } \
  }

TEST(IntP, int *);
TEST(ConstIntP, const int *);
TEST(IntRef, int &);
TEST(ConstIntRef, const int &);
TEST(VoidP, void *);
TEST(ConstVoidP, const void *);
TEST(VoidPP, void **);

struct ExampleStruct final {};

TEST(StructPtr, ExampleStruct *);
TEST(ConstStructPtr, const ExampleStruct *);
TEST(StructRef, ExampleStruct &);
TEST(ConstStructRef, const ExampleStruct &);

#undef TEST

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_INFERRED_LIFETIMES_H_
