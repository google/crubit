// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_

#pragma clang lifetime_elision

struct TestStruct1 {
  int i;
};

struct TestStruct2 {
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

  int i;
};

struct OperandForOutOfLineDefinition {
  // Non-`inline` definition.  Should generate:
  // impl PartialEq for TestStructForOutOfLineDefinition
  bool operator==(const OperandForOutOfLineDefinition& other) const;
  int i;
};

struct OperandForFreeFunc {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for TestStructForFreeFunc.
bool operator==(const OperandForFreeFunc& lhs, const OperandForFreeFunc& rhs);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_OPERATORS_OPERATORS_H_
