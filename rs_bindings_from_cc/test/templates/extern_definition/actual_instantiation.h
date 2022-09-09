// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXTERN_DEFINITION_ACTUAL_INSTANTIATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXTERN_DEFINITION_ACTUAL_INSTANTIATION_H_

#include "rs_bindings_from_cc/test/templates/extern_definition/extern_definition.h"

#pragma clang lifetime_elision

namespace actual_instantiation_ns {

using MyTypeAlias = extern_definition::MyTemplate<int>;

}  // namespace actual_instantiation_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_EXTERN_DEFINITION_ACTUAL_INSTANTIATION_H_
