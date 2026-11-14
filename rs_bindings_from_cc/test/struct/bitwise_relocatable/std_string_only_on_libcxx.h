// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX_H_

// Tests `CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX` in a header that
// includes no other standard library header. (With header modules,
// `rs_bindings_from_cc` used to not detect libc++ here.)

#include <string>

#include "support/annotations.h"

struct StdStringOnlyOnLibcxx {
  CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX std::string s;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX_H_
