// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_CMAKE_TEST_UNBOUND_DEP_TOP_H_
#define CRUBIT_CMAKE_TEST_UNBOUND_DEP_TOP_H_

#include "dep.h"

int Visible();
int UsesHidden(Hidden h);

#endif  // CRUBIT_CMAKE_TEST_UNBOUND_DEP_TOP_H_
