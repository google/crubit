// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_ENUM_FORWARD_DECLARATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_ENUM_FORWARD_DECLARATION_H_

// Duplicate forward declarations, in the same header.
enum A : int;
enum A : int;

// `A` is defined later in this header, and `Func` is NOT inline, therefore,
// Crubit should be able to generate bindings for `Func`.
void Func(A);

// The definition.
enum A : int {
  kA = 1,
};

// Forward declaration that comes after the definition.
enum A : int;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_ENUM_FORWARD_DECLARATION_H_
