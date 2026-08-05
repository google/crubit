// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DEBUG_DERIVED_DEBUGGABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DEBUG_DERIVED_DEBUGGABLES_H_

#include "support/annotations.h"

struct CRUBIT_MUST_BIND Base {};

struct CRUBIT_MUST_BIND PubliclyDerived : Base {};
struct CRUBIT_MUST_BIND PrivatelyDerived : private Base {};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DEBUG_DERIVED_DEBUGGABLES_H_
