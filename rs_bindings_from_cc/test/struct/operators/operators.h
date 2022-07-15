// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_

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
#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_
