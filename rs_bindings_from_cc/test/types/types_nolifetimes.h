// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <type_traits>

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NOLIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NOLIFETIMES_H_

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

TEST(FuncRef, std::type_identity_t<void()> &);
TEST(FuncPtr, std::type_identity_t<void()> *);
TEST(UnsafeFuncRef, std::type_identity_t<void(void *)> &);
TEST(UnsafeFuncPtr, std::type_identity_t<void(void *)> *);

template <uint8_t default_value>
struct IntTemplateStruct {
  uint8_t field = default_value;
};

TEST(IntTemplate, IntTemplateStruct<1>);
TEST(IntTemplateCharEscape, IntTemplateStruct<'\f'>);

#undef TEST

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NOLIFETIMES_H_
