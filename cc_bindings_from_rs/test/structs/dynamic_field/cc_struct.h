// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_DYNAMIC_FIELD_CC_STRUCT_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_DYNAMIC_FIELD_CC_STRUCT_H_

class DynamicWithVirtualDtor {
 public:
  DynamicWithVirtualDtor() = default;
  DynamicWithVirtualDtor(const DynamicWithVirtualDtor&) = delete;
  virtual ~DynamicWithVirtualDtor() = default;
  virtual int value() const { return 42; }
};

class DynamicMovable {
 public:
  DynamicMovable() = default;
  DynamicMovable(DynamicMovable&&) = default;
  virtual ~DynamicMovable() = default;
  virtual int value() const { return 7; }
};

class DynamicNoDtor final {
 public:
  DynamicNoDtor() = default;
  DynamicNoDtor(const DynamicNoDtor&) = delete;
  virtual int value() const { return 99; }
};

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_DYNAMIC_FIELD_CC_STRUCT_H_
