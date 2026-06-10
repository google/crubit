// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_TYPE_ALIAS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_TYPE_ALIAS_H_

#include "rs_bindings_from_cc/test/assume_lifetimes/type_alias_target_without_assume_lifetimes.h"

struct TypeAliasCtor {
  explicit TypeAliasCtor(MyStringView a);
  ~TypeAliasCtor();
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_TYPE_ALIAS_H_
