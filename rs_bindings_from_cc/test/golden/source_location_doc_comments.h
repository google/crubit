// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOCATION_DOC_COMMENTS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOCATION_DOC_COMMENTS_H_

#include "rs_bindings_from_cc/test/golden/source_location_doc_comments_macro_def.h"

/// A comment immediate above the macro expansion.
STRUCT_MACRO_FROM_A_DIFFERENT_HEADER(int);

#define STRUCT_MACRO_FROM_THIS_HEADER(field_name)          \
  STRUCT ECHO(SomeStruct) {                                \
    /** A comment on a field of macro-generated struct. */ \
    int field_name;                                        \
  };

STRUCT_MACRO_FROM_THIS_HEADER(some_field);

#define STRUCT_MACRO_HELPER(struct_name)        \
  /** A comment on a macro-generated struct. */ \
  struct struct_name {
#define STRUCT_MACRO_TRANSITIVELY_CALLING_OTHER_MACROS(struct_name) \
  STRUCT_MACRO_HELPER(struct_name)                                  \
  }

/// A doc comment on SomeStruct3 immediately above the macro expansion.
STRUCT_MACRO_TRANSITIVELY_CALLING_OTHER_MACROS(SomeStruct3);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOCATION_DOC_COMMENTS_H_
