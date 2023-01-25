// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOC_DOC_COMMENTS_MACRO_DEF_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOC_DOC_COMMENTS_MACRO_DEF_H_

#define STRUCT struct

#define ECHO(input) input

#define STRUCT_IMPL(type)                          \
  /** Throw a line of doc comment into the mix. */ \
  STRUCT ECHO(StructFromMacro) final { type val; };

#define STRUCT_MACRO_FROM_A_DIFFERENT_HEADER(type) STRUCT_IMPL(type)

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOC_DOC_COMMENTS_MACRO_DEF_H_
