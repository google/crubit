// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_BASIC_FORWARD_DECLARATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_BASIC_FORWARD_DECLARATION_H_

#pragma clang lifetime_elision

// Duplicate forward declarations, in the same header.
struct A;
struct A;

// `A` is defined later in this header, and `Func` is NOT inline, therefore,
// Crubit should be able to generate bindings for `Func`.
void Func(A);

// The definition.
struct A final {};

// Forward declaration that comes after the definition.
struct A;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_BASIC_FORWARD_DECLARATION_H_
