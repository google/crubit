// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_INHERITED_OPERATORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_INHERITED_OPERATORS_H_

#include "rs_bindings_from_cc/test/struct/operators/operator_mixins.h"

// Inherits `operator==` and `operator!=` from a base class in another target.
struct Equatable final : mixins::EqualityMixin<Equatable> {
  int key;
  int payload;
};

namespace ns {

// Inherits `operator==`, `operator!=`, and `operator<` from indirect bases in
// another target.
struct Comparable final : mixins::ComparisonMixin<Comparable> {
  int key;
  int payload;
};

}  // namespace ns

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_INHERITED_OPERATORS_H_
