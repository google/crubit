// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_BRIDGING_CLASS_TEMPLATE_INSTANTIATION2_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_BRIDGING_CLASS_TEMPLATE_INSTANTIATION2_H_

#include "rs_bindings_from_cc/test/templates/bridging/class_template_definition.h"

#pragma clang lifetime_elision

inline int GetValue(const MyTemplate<int>& t) { return t.value(); }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_BRIDGING_CLASS_TEMPLATE_INSTANTIATION2_H_
