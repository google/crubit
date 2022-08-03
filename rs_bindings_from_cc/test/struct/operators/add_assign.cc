// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/operators/add_assign.h"

AddAssignFreeByValue& operator+=(AddAssignFreeByValue& lhs,
                                 AddAssignFreeByValue rhs) {
  lhs.i += rhs.i;
  return lhs;
}

AddAssignFriendByValue& operator+=(AddAssignFriendByValue& lhs,
                                   AddAssignFriendByValue rhs) {
  lhs.i += rhs.i;
  return lhs;
}
