// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_TYPE_OWNERSHIP_DEFINITION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_TYPE_OWNERSHIP_DEFINITION_H_

#include "rs_bindings_from_cc/test/forward_declaration/type_ownership/forward_declaration.h"

#pragma clang lifetime_elision

struct ForwardDeclaredStruct final {};

inline void FuncA(A) {}
inline void FuncB(B) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_TYPE_OWNERSHIP_DEFINITION_H_
