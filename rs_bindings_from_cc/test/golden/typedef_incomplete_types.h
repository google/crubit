// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEF_INCOMPLETE_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEF_INCOMPLETE_TYPES_H_

extern "C" {
typedef struct IncompleteExternC IncompleteExternC;
}

typedef struct Incomplete Incomplete;

struct HasPointerToIncompleteTypedefs {
  IncompleteExternC* incomplete_extern_c;
  Incomplete* incomplete;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEF_INCOMPLETE_TYPES_H_
