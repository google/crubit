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

  // Comparison with the same struct.  Should generate:
  // impl PartialOrd for TestStruct2
  // `PartialOrd<TestStruct2>` also ok.
  inline bool operator<(const TestStruct2& other) const {
    return (i % 10) < (other.i % 10);
  }

  // Comparison with another struct.  Shouldn't generate anything since the
  // operands are not of the same type.
  inline bool operator<(const TestStruct1& other) const {
    return (i % 10) < (other.i % 10);
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

  // Non-`inline` definition.  Should generate:
  // impl PartialOrd for TestStructForOutOfLineDefinition
  bool operator<(const OperandForOutOfLineDefinition& other) const;

  int i;
};

//////////////////////////////////////////////////////////////////////

struct OperandForFreeFunc final {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for TestStructForFreeFunc.
bool operator==(const OperandForFreeFunc& lhs, const OperandForFreeFunc& rhs);

// Non-member function. Should generate:
// impl PartialOrd for TestStructForFreeFunc.
bool operator<(const OperandForFreeFunc& lhs, const OperandForFreeFunc& rhs);

//////////////////////////////////////////////////////////////////////

struct OperandByRef final {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for OperandByRef.
bool operator==(const OperandByRef& lhs, const OperandByRef& rhs);

// Non-member function. Should generate:
// impl PartialOrd for OperandByRef.
bool operator<(const OperandByRef& lhs, const OperandByRef& rhs);

//////////////////////////////////////////////////////////////////////

struct OperandByValue final {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for OperandByValue.
bool operator==(OperandByValue lhs, OperandByValue rhs);

// Non-member function. Should generate:
// impl PartialOrd for OperandByValue.
bool operator<(OperandByValue lhs, OperandByValue rhs);

//////////////////////////////////////////////////////////////////////

struct OperandByRefAndValue final {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for OperandByRefAndValue.
bool operator==(const OperandByRefAndValue& lhs, OperandByRefAndValue rhs);

// Non-member function. Should generate:
// impl PartialOrd for OperandByRefAndValue.
bool operator<(const OperandByRefAndValue& lhs, OperandByRefAndValue rhs);

//////////////////////////////////////////////////////////////////////

struct OperandByValueAndRef final {
  int i;
};

// Non-member function. Should generate:
// impl PartialEq for OperandByValueAndRef.
bool operator==(OperandByValueAndRef lhs, const OperandByValueAndRef& rhs);

// Non-member function. Should generate:
// impl PartialOrd for OperandByValueAndRef.
bool operator<(OperandByValueAndRef lhs, const OperandByValueAndRef& rhs);

//////////////////////////////////////////////////////////////////////

struct OperandForFreeFuncInDifferentNamespace final {
  int i;
};

namespace test_namespace_bindings {

// This should *not* generate PartialEq, because we are trying to mimic ADL.

// TODO(b/200066396): We currently generate PartialEq here, but it doesn't
// compile.
// bool operator==(const OperandForFreeFuncInDifferentNamespace& lhs,
//                 const OperandForFreeFuncInDifferentNamespace& rhs);

}  // namespace test_namespace_bindings

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
