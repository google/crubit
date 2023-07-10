// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEFS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEFS_H_

#pragma clang lifetime_elision

struct SomeStruct {
  typedef int nested_type;
};
typedef struct SomeStruct SomeStruct;

typedef struct {
} SomeOtherStruct;

union SomeUnion {};
typedef union SomeUnion SomeUnion;

typedef union {
} SomeOtherUnion;

SomeStruct::nested_type FunctionUsingNestedType();

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEFS_H_
