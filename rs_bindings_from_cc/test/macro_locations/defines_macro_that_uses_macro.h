// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_LOCATIONS_DEFINES_MACRO_THAT_USES_MACRO_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_LOCATIONS_DEFINES_MACRO_THAT_USES_MACRO_H_

#include "rs_bindings_from_cc/test/macro_locations/defines_macro.h"

#define DEFINE_STRUCT_AND_FUNCTION(type) \
  DEFINE_STRUCT(type)                    \
  inline type functionFromMacro(type x) { return x; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_MACRO_LOCATIONS_DEFINES_MACRO_THAT_USES_MACRO_H_
