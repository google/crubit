// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DECLARATION_2_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DECLARATION_2_H_

#pragma clang lifetime_elision

struct UnpinStruct;
struct NonunpinStruct;

int ReadUnpinStruct(const UnpinStruct& s);
void WriteUnpinStruct(UnpinStruct& s, int value);

int ReadNonunpinStruct(const NonunpinStruct& s);
void WriteNonunpinStruct(NonunpinStruct& s, int value);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FORWARD_DECLARATIONS_DECLARATION_2_H_
