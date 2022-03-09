// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_

#pragma clang lifetime_elision

struct DestructionOrderRecorder {
  ~DestructionOrderRecorder() { RecordDestruction(int_field); }
  int int_field;

  static void RecordDestruction(int int_field);
  static int GetDestructionRecord();
  static void ClearDestructionRecord();
};

struct FieldDestructionOrderTester {
  DestructionOrderRecorder field1;
  DestructionOrderRecorder field2;
  DestructionOrderRecorder field3;

  static void DestructFromCpp(int field1, int field2, int field3) {
    FieldDestructionOrderTester tester = {
        .field1 = DestructionOrderRecorder{.int_field = field1},
        .field2 = DestructionOrderRecorder{.int_field = field2},
        .field3 = DestructionOrderRecorder{.int_field = field3},
    };
  }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_DESTRUCTORS_FIELD_DESTRUCTION_ORDER_H_
