// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_

#include <cstdint>

#pragma clang lifetime_elision

class AddableConstMember final {
 public:
  AddableConstMember operator+(const AddableConstMember& rhs) const;

 private:
  int field_;
};

class AddableNonConstMember final {
 public:
  AddableNonConstMember operator+(const AddableNonConstMember& rhs);

 private:
  int field_;
};

class AddableFriend final {
 public:
  friend AddableFriend operator+(const AddableFriend& lhs,
                                 const AddableFriend& rhs);

 private:
  int field_;
};

class AddableFree final {};
AddableFree operator+(const AddableFree& lhs, const AddableFree& rhs);
AddableFree operator+(AddableFree& lhs, AddableFree& rhs);
AddableFree operator+(AddableFree lhs, AddableFree rhs);
AddableFree operator+(AddableFree&& lhs, AddableFree rhs);

class Overloaded final {};
int operator+(const Overloaded& lhs, std::int16_t rhs);
int operator+(const Overloaded& lhs, std::int32_t rhs);

class IncompatibleLHS final {};
IncompatibleLHS operator+(int lhs, const IncompatibleLHS& rhs);
IncompatibleLHS operator+(int& lhs, const IncompatibleLHS& rhs);

class AddableReturnsVoid final {
 public:
  void operator+(const AddableReturnsVoid& rhs) const;

 private:
  int field_;
};

class AddableConstMemberNonunpin final {
 public:
  AddableConstMemberNonunpin operator+(
      const AddableConstMemberNonunpin& rhs) const;
  ~AddableConstMemberNonunpin() {}

 private:
  int field_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_
