// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_INLINE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_INLINE_H_

#include "rs_bindings_from_cc/test/namespace/reopened/original_namespace.h"

namespace foo {

inline SomeStruct FunctionUsesNamespaceType() { return SomeStruct(); }

inline int Returns42() { return 42; }

}  // namespace foo

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_INLINE_H_
