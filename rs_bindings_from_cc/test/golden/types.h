// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

#pragma clang lifetime_elision

// To avoid relying on cstdef
using PtrDiff =
    decltype(static_cast<int*>(nullptr) - static_cast<int*>(nullptr));
using Size = decltype(sizeof(0));

struct SomeStruct final {};

struct ForwardDeclaredStruct;

struct FieldTypeTestStruct final {
  bool bool_field;
  char char_field;

  unsigned char unsigned_char_field;
  signed char signed_char_field;
  char16_t char16_t_field;
  char32_t char32_t_field;
  wchar_t wchar_t_field;

  short short_field;
  int int_field;
  long long_field;
  long long long_long_field;

  unsigned short unsigned_short_field;
  unsigned int unsigned_int_field;
  unsigned long unsigned_long_field;
  unsigned long long unsigned_long_long_field;

  signed short signed_short_field;
  signed int signed_int_field;
  signed long signed_long_field;
  signed long long signed_long_long_field;

  PtrDiff ptrdiff_t_field;
  Size size_t_field;

  float float_field;
  double double_field;

  int* ptr_field;
  void* void_ptr_field;
  const void* const_void_ptr_field;
  void** void_double_ptr_field;

  SomeStruct struct_field;
  SomeStruct* struct_ptr_field;
  const SomeStruct* const_struct_ptr_field;
  SomeStruct& struct_ref_field;
  const SomeStruct& const_struct_ref_field;
  // TODO(b/226580208): Uncomment when these don't cause struct import to fail.
  // SomeStruct&& struct_rvalue_ref_field;
  // const SomeStruct&& const_struct_rvalue_ref_field;

  ForwardDeclaredStruct* forward_declared_ptr_field;
};

inline void VoidReturningFunction() {}

// Note especially the use of references. If we convert those to pointers,
// this becomes un-compilable. The syntax here is awful, but this is a function
// returning a function. In ML-like syntax:
// FunctionPointerReturningFunction : () -> (const int&, int*) -> int&
inline int& (*FunctionPointerReturningFunction())(const int&, int*) {
  return nullptr;
}

inline void* FunctionWithVoidPointers(void*, const void*) { return nullptr; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_
