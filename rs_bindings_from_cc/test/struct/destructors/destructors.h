// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_

#include <utility>

class [[clang::trivial_abi]] DestructionOrderRecorder final {
 public:
  explicit DestructionOrderRecorder(int int_field) : int_field_(int_field) {}
  DestructionOrderRecorder(const DestructionOrderRecorder& other) = delete;
  DestructionOrderRecorder& operator=(const DestructionOrderRecorder& other) =
      delete;
  DestructionOrderRecorder(DestructionOrderRecorder&& other)
      : int_field_(other.int_field_) {
    other.int_field_ = 0;
  }
  DestructionOrderRecorder& operator=(DestructionOrderRecorder&& other) {
    if (int_field_ != 0) {
      RecordDestruction(int_field_);
    }
    int_field_ = other.int_field_;
    other.int_field_ = 0;
    return *this;
  }
  ~DestructionOrderRecorder() {
    if (int_field_ != 0) {
      RecordDestruction(int_field_);
    }
  }

  static void RecordDestruction(int int_field);
  static int GetDestructionRecord();
  static void ClearDestructionRecord();

 private:
  int int_field_;
};

class [[clang::trivial_abi]] FieldDestructionOrderTester final {
 public:
  FieldDestructionOrderTester(DestructionOrderRecorder field1,
                              DestructionOrderRecorder field2,
                              DestructionOrderRecorder field3)
      : field1_(std::move(field1)),
        field2_(std::move(field2)),
        field3_(std::move(field3)) {}

  static void DestructFromCpp(int field1, int field2, int field3) {
    auto tester = FieldDestructionOrderTester(DestructionOrderRecorder(field1),
                                              DestructionOrderRecorder(field2),
                                              DestructionOrderRecorder(field3));
  }

 private:
  DestructionOrderRecorder field1_;
  DestructionOrderRecorder field2_;
  DestructionOrderRecorder field3_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_
