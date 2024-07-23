// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_ORIGINAL_NAMESPACE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_ORIGINAL_NAMESPACE_H_

// Namespaces can be opened in one header with Crubit disabled,
// and then _reopened_ in another header with Crubit enabled.
// See b/333737712
namespace foo {
struct SomeStruct {};
}  // namespace foo

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_ORIGINAL_NAMESPACE_H_
