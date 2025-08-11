// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DISABLE_DISABLE_HEADER_HEADER2_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DISABLE_DISABLE_HEADER_HEADER2_H_

// NOLINTNEXTLINE
#include "rs_bindings_from_cc/test/disable/disable_header/disabled_header.h"

inline int bar() { return 42; }

// the functions/etc. defined in disabled_header.h should not be assigned to
// this header just because disabled_header.h is included but didn't get
// bindings!

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DISABLE_DISABLE_HEADER_HEADER2_H_
