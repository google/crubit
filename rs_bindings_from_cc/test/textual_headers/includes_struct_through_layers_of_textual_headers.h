// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEXTUAL_HEADERS_INCLUDES_STRUCT_THROUGH_LAYERS_OF_TEXTUAL_HEADERS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEXTUAL_HEADERS_INCLUDES_STRUCT_THROUGH_LAYERS_OF_TEXTUAL_HEADERS_H_

#include "rs_bindings_from_cc/test/textual_headers/includes_textual_header.inc"

inline int getValue(MyStruct s) { return s.value; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEXTUAL_HEADERS_INCLUDES_STRUCT_THROUGH_LAYERS_OF_TEXTUAL_HEADERS_H_
