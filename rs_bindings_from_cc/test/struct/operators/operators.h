// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_

#include <cstdint>

#pragma clang lifetime_elision

struct TestStruct1 final {
  int i;
};

struct TestStruct2 final {
  // Comparison with the same struct.  Should generate:
  // impl PartialEq for TestStruct2.  // `PartialEq<TestStruct2>` also ok.
  inline bool operator==(const TestStruct2& other) const {
    return (i % 10) == (other.i % 10);
  }

  // Comparison with another struct.  Should generate:
  // impl PartialEq<TestStruct1> for TestStruct2.
  inline bool operator==(const TestStruct1& other) const {
    return (i % 10) == (other.i % 10);
  }

  // Test that method names starting with "operator" are not confused with real
  // operator names (e.g. accidentally treating "operator1" as an unrecognized /
  // unsupported operator).
  inline int operator1() const { return i; }

  int i;
};

//////////////////////////////////////////////////////////////////////

struct OperandForOutOfLineDefinition final {
  // Non-`inline` definition.  Should generate:
  // impl PartialEq for TestStructForOutOfLineDefinition
  bool operator==(const OperandForOutOfLineDefinition& other) const;
  int i;
};

//////////////////////////////////////////////////////////////////////

struct OperandForFreeFunc final {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for TestStructForFreeFunc.
bool operator==(const OperandForFreeFunc& lhs, const OperandForFreeFunc& rhs);

//////////////////////////////////////////////////////////////////////

struct OperandForFreeFuncInDifferentNamespace final {
  int i;
};

namespace some_other_namespace {

// This should *not* generate PartialEq, because we are trying to mimic ADL.
bool operator==(const OperandForFreeFuncInDifferentNamespace& lhs,
                const OperandForFreeFuncInDifferentNamespace& rhs);

}  // namespace some_other_namespace

//////////////////////////////////////////////////////////////////////

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

//////////////////////////////////////////////////////////////////////

struct ManyOperators final {
  ManyOperators operator+() const { return ManyOperators{+i}; }
  ManyOperators operator-() const { return ManyOperators{-i}; }
  ManyOperators operator!() const { return ManyOperators{!i}; }
  ManyOperators operator~() const { return ManyOperators{~i}; }
  ManyOperators operator+(const ManyOperators rhs) const {
    return ManyOperators{i + rhs.i};
  }
  ManyOperators operator-(const ManyOperators rhs) const {
    return ManyOperators{i - rhs.i};
  }
  ManyOperators operator*(const ManyOperators rhs) const {
    return ManyOperators{i * rhs.i};
  }
  ManyOperators operator/(const ManyOperators rhs) const {
    return ManyOperators{i / rhs.i};
  }
  ManyOperators operator%(const ManyOperators rhs) const {
    return ManyOperators{i % rhs.i};
  }
  ManyOperators operator&(const ManyOperators rhs) const {
    return ManyOperators{i & rhs.i};
  }
  ManyOperators operator|(const ManyOperators rhs) const {
    return ManyOperators{i | rhs.i};
  }
  ManyOperators operator^(const ManyOperators rhs) const {
    return ManyOperators{i ^ rhs.i};
  }
  ManyOperators operator<<(const ManyOperators rhs) const {
    return ManyOperators{i << rhs.i};
  }
  ManyOperators operator>>(const ManyOperators rhs) const {
    return ManyOperators{i >> rhs.i};
  }

  ManyOperators& operator+=(const ManyOperators rhs) {
    i += rhs.i;
    return *this;
  }
  ManyOperators& operator-=(const ManyOperators rhs) {
    i -= rhs.i;
    return *this;
  }
  ManyOperators& operator*=(const ManyOperators rhs) {
    i *= rhs.i;
    return *this;
  }
  ManyOperators& operator/=(const ManyOperators rhs) {
    i /= rhs.i;
    return *this;
  }
  ManyOperators& operator%=(const ManyOperators rhs) {
    i %= rhs.i;
    return *this;
  }
  ManyOperators& operator&=(const ManyOperators rhs) {
    i &= rhs.i;
    return *this;
  }
  ManyOperators& operator|=(const ManyOperators rhs) {
    i |= rhs.i;
    return *this;
  }
  ManyOperators& operator^=(const ManyOperators rhs) {
    i ^= rhs.i;
    return *this;
  }
  ManyOperators& operator<<=(const ManyOperators rhs) {
    i <<= rhs.i;
    return *this;
  }
  ManyOperators& operator>>=(const ManyOperators rhs) {
    i >>= rhs.i;
    return *this;
  }

  int i;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_
