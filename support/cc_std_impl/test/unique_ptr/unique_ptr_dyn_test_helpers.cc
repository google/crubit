// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/cc_std_impl/test/unique_ptr/unique_ptr_dyn_test_helpers.h"

namespace unique_ptr_dyn_test {

static int instances_count = 0;

int BaseWithVirtualDestructor::instances() { return instances_count; }

void BaseWithVirtualDestructor::set_instances(int n) { instances_count = n; }

BaseWithVirtualDestructor::~BaseWithVirtualDestructor() { --instances_count; }

Derived::~Derived() {}

static bool was_delete_called = false;

bool WithOverloadedDelete::delete_called() { return was_delete_called; }

void WithOverloadedDelete::set_delete_called(bool b) { was_delete_called = b; }

void WithOverloadedDelete::operator delete(void* p) {
  was_delete_called = true;
  ::operator delete(p);
}

}  // namespace unique_ptr_dyn_test
