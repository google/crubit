// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_H_

#include <cstdint>

#pragma clang lifetime_elision

struct AddableConstMemberInt final {
  // impl Add<i32> for &AddableConstMemberInt { type Output = i32; .. }
  int operator+(int rhs) const { return i + rhs; }

  int i;
};

struct AddableConstMemberByRef final {
  // impl Add<&AddableConstMemberByRef> for &AddableConstMemberByRef {
  //     type Output = AddableConstMemberByRef;
  //     ..
  // }
  AddableConstMemberByRef operator+(const AddableConstMemberByRef& rhs) const {
    return AddableConstMemberByRef{i + rhs.i};
  }

  int i;
};

struct AddableNonConstMemberByRef final {
  // impl Add<&AddableNonConstMemberByRef> for &mut AddableNonConstMemberByRef {
  //     type Output = AddableNonConstMemberByRef;
  //     ..
  // }
  AddableNonConstMemberByRef operator+(const AddableNonConstMemberByRef& rhs) {
    return AddableNonConstMemberByRef{i + rhs.i};
  }

  int i;
};

struct AddableConstMemberByValue final {
  // impl Add<AddableConstMemberByValue> for &AddableConstMemberByValue {
  //     type Output = AddableConstMemberByValue;
  //     ..
  // }
  AddableConstMemberByValue operator+(AddableConstMemberByValue rhs) const {
    return AddableConstMemberByValue{i + rhs.i};
  }

  int i;
};

struct AddableNonConstMemberByValue final {
  // impl Add<AddableNonConstMemberByValue> for &AddableNonConstMemberByValue {
  //     type Output = AddableNonConstMemberByValue;
  //     ..
  // }
  AddableNonConstMemberByValue operator+(AddableNonConstMemberByValue rhs) {
    return AddableNonConstMemberByValue{i + rhs.i};
  }

  int i;
};

struct AddableReturnsVoid final {
  void operator+(const AddableReturnsVoid& rhs) { i += rhs.i; }

  int i;
};

struct AddableReturnsNontrivial final {
  ~AddableReturnsNontrivial() {}
  AddableReturnsNontrivial operator+(
      const AddableReturnsNontrivial& rhs) const {
    return AddableReturnsNontrivial{i + rhs.i};
  }

  int i;
};

struct UnpinStruct final {
  int i;
};

// impl Add<&UnpinStruct> for &UnpinStruct {
//     type Output = UnpinStruct;
//     ..
// }
UnpinStruct operator+(const UnpinStruct& lhs, const UnpinStruct& rhs);

// impl Add<&mut UnpinStruct> for &mut UnpinStruct {
//     type Output = UnpinStruct;
//     ..
// }
UnpinStruct operator+(UnpinStruct& lhs, UnpinStruct& rhs);

// impl Add<UnpinStruct> for UnpinStruct {
//     type Output = UnpinStruct;
//     ..
// }
UnpinStruct operator+(UnpinStruct lhs, UnpinStruct rhs);

struct AddableOverloaded final {
  char int16_char;
  char int32_char;
};

char operator+(AddableOverloaded lhs, std::int16_t rhs);
char operator+(AddableOverloaded lhs, std::int32_t rhs);

struct AddableFriendByConstRef final {
  friend AddableFriendByConstRef operator+(const AddableFriendByConstRef& lhs,
                                           const AddableFriendByConstRef& rhs) {
    return AddableFriendByConstRef{lhs.i + rhs.i};
  }

  int i;
};

struct AddableFriendByRef final {
  friend AddableFriendByRef operator+(AddableFriendByRef& lhs,
                                      AddableFriendByRef& rhs) {
    return AddableFriendByRef{lhs.i + rhs.i};
  }

  int i;
};

struct AddableFriendByValue final {
  friend AddableFriendByValue operator+(AddableFriendByValue lhs,
                                        AddableFriendByValue rhs) {
    return AddableFriendByValue{lhs.i + rhs.i};
  }

  int i;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_H_
