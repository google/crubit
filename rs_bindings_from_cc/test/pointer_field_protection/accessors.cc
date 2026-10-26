// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/pointer_field_protection/accessors.h"

namespace {
int g_value = PointerHolder::kValue;
}  // namespace

int PointerHolder::GetFromCpp() const { return *ptr_; }

void PointerHolder::SetFromCpp() { ptr_ = &g_value; }
