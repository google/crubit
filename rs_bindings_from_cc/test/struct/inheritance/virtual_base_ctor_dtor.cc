// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/inheritance/virtual_base_ctor_dtor.h"

namespace virtual_base_ctor_dtor {

namespace {
int g_virtual_base_destructor_call_count = 0;
int g_destructor_call_count = 0;
}  // namespace

VirtualBase::~VirtualBase() { g_virtual_base_destructor_call_count++; }

// static
int VirtualBase::GetDestructorCallCount() {
  return g_virtual_base_destructor_call_count;
}

Derived::Derived(int value) : value_(value) {}

Derived::~Derived() { g_destructor_call_count++; }

int Derived::value() const { return value_; }

int Derived::base_field_from_cpp() const { return base_field; }

// static
int Derived::GetDestructorCallCount() { return g_destructor_call_count; }

}  // namespace virtual_base_ctor_dtor
