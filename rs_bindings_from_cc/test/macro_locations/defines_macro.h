// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_LOCATIONS_DEFINES_MACRO_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_LOCATIONS_DEFINES_MACRO_H_

#define DEFINE_STRUCT(type)      \
  struct StructFromMacro final { \
    type val;                    \
  };

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_LOCATIONS_DEFINES_MACRO_H_
