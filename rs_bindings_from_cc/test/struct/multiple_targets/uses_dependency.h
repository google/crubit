// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_MULTIPLE_TARGETS_USES_DEPENDENCY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_MULTIPLE_TARGETS_USES_DEPENDENCY_H_
#include "rs_bindings_from_cc/test/struct/multiple_targets/dependency.h"

#pragma clang lifetime_elision

inline Dependency UseDependency(Dependency dependency) { return dependency; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_MULTIPLE_TARGETS_USES_DEPENDENCY_H_
