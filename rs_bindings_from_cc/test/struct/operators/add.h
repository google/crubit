// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_H_

#include <cstdint>

#pragma clang lifetime_elision

inline void IntentionallyNontrivial() {}

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

struct AddableNontrivialByValue final {
  ~AddableNontrivialByValue() { IntentionallyNontrivial(); }
  AddableNontrivialByValue operator+(AddableNontrivialByValue rhs) const {
    return AddableNontrivialByValue{i + rhs.i};
  }

  int i;
};

struct UnpinStructByConstRef final {
  int i;
};

struct UnpinStructByMutRef final {
  int i;
};

struct UnpinStructByValue final {
  int i;
};

// impl Add<&UnpinStructByConstRef> for &UnpinStructByConstRef {
//     type Output = UnpinStructByConstRef;
//     ..
// }
UnpinStructByConstRef operator+(const UnpinStructByConstRef& lhs,
                                const UnpinStructByConstRef& rhs);

// impl Add<&mut UnpinStructByMutRef> for &mut UnpinStructByMutRef {
//     type Output = UnpinStructByMutRef;
//     ..
// }
UnpinStructByMutRef operator+(UnpinStructByMutRef& lhs,
                              UnpinStructByMutRef& rhs);

// impl Add<UnpinStructByValue> for UnpinStructByValue {
//     type Output = UnpinStructByValue;
//     ..
// }
UnpinStructByValue operator+(UnpinStructByValue lhs, UnpinStructByValue rhs);

struct AddableOverloaded final {
  unsigned char int16_char;
  unsigned char int32_char;
};

unsigned char operator+(AddableOverloaded lhs, std::int16_t rhs);
unsigned char operator+(AddableOverloaded lhs, std::int32_t rhs);

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

struct AddableReturnsNontrivial final {
  ~AddableReturnsNontrivial() { IntentionallyNontrivial(); }
  AddableReturnsNontrivial operator+(
      const AddableReturnsNontrivial& rhs) const {
    return AddableReturnsNontrivial{i + rhs.i};
  }

  int i;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_ADD_H_
