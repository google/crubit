// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/pointer_field_protection/pointer_field_protection.h"

namespace {
int g_value = TrivialAbiPointer::kValue;
}  // namespace

TrivialAbiPointer TrivialAbiPointer::Create() {
  return TrivialAbiPointer(&g_value);
}

int TrivialAbiPointer::Get() const { return *ptr_; }

int TrivialAbiPointer::GetAfterCppCopy() {
  TrivialAbiPointer local = Create();
  // No `std::make_unique`: this library is built with different PFP flags
  // than the C++ standard library.
  TrivialAbiPointer* heap = new TrivialAbiPointer(local);
  int result = heap->Get();
  delete heap;
  return result;
}
