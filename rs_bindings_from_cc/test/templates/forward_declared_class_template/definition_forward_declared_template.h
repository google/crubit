// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FORWARD_DECLARED_CLASS_TEMPLATE_DEFINITION_FORWARD_DECLARED_TEMPLATE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FORWARD_DECLARED_CLASS_TEMPLATE_DEFINITION_FORWARD_DECLARED_TEMPLATE_H_

#include "rs_bindings_from_cc/test/templates/forward_declared_class_template/use_forward_declared_template.h"

#pragma clang lifetime_elision

template <typename T>
struct ForwardDeclaredTemplateStruct final {};

inline void Func(B) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FORWARD_DECLARED_CLASS_TEMPLATE_DEFINITION_FORWARD_DECLARED_TEMPLATE_H_
