// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_INCLUDED_BEFORE_DEFINITION_DEFINITION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_INCLUDED_BEFORE_DEFINITION_DEFINITION_H_

#include "rs_bindings_from_cc/test/forward_declaration/included_before_definition/forward_declaration1.h"  // IWYU pragma: keep
#include "rs_bindings_from_cc/test/forward_declaration/included_before_definition/forward_declaration2.h"  // IWYU pragma: keep

class A final {};

namespace my_namespace {
class B final {};
}  // namespace my_namespace

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_INCLUDED_BEFORE_DEFINITION_DEFINITION_H_
