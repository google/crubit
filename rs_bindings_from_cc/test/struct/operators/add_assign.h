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

// This tests the following aspect of
// https://en.cppreference.com/w/cpp/language/adl: "ADL can find a friend
// function (typically, an overloaded operator) that is defined entirely within
// a class or class template, even if it was never declared at namespace level."
struct AddAssignFriendByValue final {
  friend AddAssignFriendByValue& operator+=(AddAssignFriendByValue& lhs,
                                            AddAssignFriendByValue rhs);

  int i;
};

struct AddAssignFreeByValue final {
  int i;
};

AddAssignFreeByValue& operator+=(AddAssignFreeByValue& lhs,
                                 AddAssignFreeByValue rhs);

// This provides additional test coverage for b/244311755.
struct AddAssignInlineFriendByValue final {
  friend inline AddAssignInlineFriendByValue& operator+=(
      AddAssignInlineFriendByValue& lhs, AddAssignInlineFriendByValue rhs);

  int i;
};

inline AddAssignInlineFriendByValue& operator+=(
    AddAssignInlineFriendByValue& lhs, AddAssignInlineFriendByValue rhs) {
  lhs.i += rhs.i;
  return lhs;
}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_ASSIGN_H_
