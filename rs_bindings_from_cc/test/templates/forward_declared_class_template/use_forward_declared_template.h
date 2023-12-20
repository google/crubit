// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FORWARD_DECLARED_CLASS_TEMPLATE_USE_FORWARD_DECLARED_TEMPLATE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FORWARD_DECLARED_CLASS_TEMPLATE_USE_FORWARD_DECLARED_TEMPLATE_H_

template <typename T>
struct TemplateWithFullDefinition final {
  // We need a field to depend on T to introduce the "implicit  instantiation of
  // undefined template 'ForwardDeclaredTemplateStruct<int>'" error.
  T field;
};

// This is a reproducer for b/274663418: A header forward-declares a template
// (which is defined in a different translation unit), and uses the
// forward-declared template in typedefs.
template <typename T>
struct ForwardDeclaredTemplateStruct;

using A = ForwardDeclaredTemplateStruct<int>;
using B = TemplateWithFullDefinition<A>;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FORWARD_DECLARED_CLASS_TEMPLATE_USE_FORWARD_DECLARED_TEMPLATE_H_
