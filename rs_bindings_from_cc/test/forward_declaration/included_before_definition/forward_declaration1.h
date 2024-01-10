// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_INCLUDED_BEFORE_DEFINITION_FORWARD_DECLARATION1_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_INCLUDED_BEFORE_DEFINITION_FORWARD_DECLARATION1_H_

class A;

// Add an example that a forward declaration nested in a namespace, because
// Crubit has special logic for "top-level" items, such as `class A`, and
// `class B` is not considered as a 'top-level' item.
namespace my_namespace {
class B;
}  // namespace my_namespace

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_INCLUDED_BEFORE_DEFINITION_FORWARD_DECLARATION1_H_
