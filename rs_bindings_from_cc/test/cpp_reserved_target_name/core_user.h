// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CPP_RESERVED_TARGET_NAME_CORE_USER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CPP_RESERVED_TARGET_NAME_CORE_USER_H_

#include "rs_bindings_from_cc/test/cpp_reserved_target_name/core.h"

#pragma clang lifetime_elision

struct StructInHeaderThatIncludeCoreHeader final {
  StructInCore struct_in_core;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CPP_RESERVED_TARGET_NAME_CORE_USER_H_
