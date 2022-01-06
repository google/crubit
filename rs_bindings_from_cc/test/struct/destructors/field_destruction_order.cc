// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/destructors/field_destruction_order.h"

namespace {
int g_destruction_record = 0;
}  // namespace

// static
void DestructionOrderRecorder::RecordDestruction(int int_field) {
  g_destruction_record = g_destruction_record * 10 + int_field;
}

// static
int DestructionOrderRecorder::GetDestructionRecord() {
  return g_destruction_record;
}

// static
void DestructionOrderRecorder::ClearDestructionRecord() {
  g_destruction_record = 0;
}
