// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_ONLY_FUNCTIONS_USING_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_ONLY_FUNCTIONS_USING_TYPES_H_

#include "rs_bindings_from_cc/test/types_only/types_only.h"

inline Copyable PassCopyable(Copyable copyable) { return copyable; }
inline Cloneable PassCloneable(Cloneable cloneable) { return cloneable; }
inline Movable PassMovable(Movable movable) { return movable; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_ONLY_FUNCTIONS_USING_TYPES_H_
