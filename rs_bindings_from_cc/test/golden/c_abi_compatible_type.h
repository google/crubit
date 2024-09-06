// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_C_ABI_COMPATIBLE_TYPE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_C_ABI_COMPATIBLE_TYPE_H_

struct [[clang::annotate("crubit_internal_rust_type", "i8")]] [[clang::annotate(
    "crubit_internal_same_abi")]] MyI8 {
  unsigned char field;
};

struct X {
  int a;
};

MyI8 ffi(MyI8 a, X b);

typedef int MyTypedefDecl;

inline void f(MyTypedefDecl a, void* b, int c) {}
#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_C_ABI_COMPATIBLE_TYPE_H_
