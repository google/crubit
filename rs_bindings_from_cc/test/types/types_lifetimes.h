// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_LIFETIMES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_LIFETIMES_H_

// Not a template, so that it isn't visible to the bindings generator.
// We're just here to save typing.
#define TEST(Name, T)                            \
  struct Name {                                  \
    T field;                                     \
    static T Function(T param) { return param; } \
  }

#define $static [[clang::annotate_type("lifetime", "static")]]

TEST(IntP, int* $static);
TEST(ConstIntP, const int* $static);
TEST(IntRef, int& $static);
TEST(ConstIntRef, const int& $static);
TEST(VoidP, void* $static);
TEST(ConstVoidP, const void* $static);
TEST(VoidPP, void* $static* $static);

struct ExampleStruct final {};

TEST(StructPtr, ExampleStruct* $static);
TEST(ConstStructPtr, const ExampleStruct* $static);
TEST(StructRef, ExampleStruct& $static);
TEST(ConstStructRef, const ExampleStruct& $static);

#undef TEST

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_LIFETIMES_H_
