// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_FORWARD_DECLARATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_FORWARD_DECLARATION_H_

struct ForwardDeclaredStruct;

// Intentionally forward declare this struct again, to ensure Crubit can handle
// it.
struct ForwardDeclaredStruct;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_FORWARD_DECLARATION_H_
