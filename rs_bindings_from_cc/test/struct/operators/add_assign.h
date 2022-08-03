// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_ASSIGN_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_ASSIGN_H_

#include <cstdint>

#pragma clang lifetime_elision

struct AddAssignMemberInt final {
  int operator+=(int rhs) {
    this->i += rhs;
    return i;
  }

  int i;
};

struct AddAssignMemberByValue final {
  AddAssignMemberByValue& operator+=(AddAssignMemberByValue rhs) {
    this->i += rhs.i;
    return *this;
  }

  int i;
};

struct AddAssignMemberByRef final {
  AddAssignMemberByRef& operator+=(AddAssignMemberByRef& rhs) {
    this->i += rhs.i;
    return *this;
  }

  int i;
};

struct AddAssignMemberByConstRef final {
  AddAssignMemberByConstRef& operator+=(const AddAssignMemberByConstRef& rhs) {
    this->i += rhs.i;
    return *this;
  }

  int i;
};

struct AddAssignFreeByValue final {
  friend AddAssignFreeByValue& operator+=(AddAssignFreeByValue& lhs,
                                          AddAssignFreeByValue rhs);

  int i;
};

struct AddAssignFriendByValue final {
  int i;
};

AddAssignFriendByValue& operator+=(AddAssignFriendByValue& lhs,
                                   AddAssignFriendByValue rhs);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_ASSIGN_H_
