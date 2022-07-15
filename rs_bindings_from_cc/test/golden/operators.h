// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_

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

class AddableReturnsVoid final {
 public:
  void operator+(const AddableReturnsVoid& rhs) const;

 private:
  int field_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_
