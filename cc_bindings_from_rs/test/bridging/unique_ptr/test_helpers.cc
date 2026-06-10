// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/unique_ptr/test_helpers.h"

#include <memory>

namespace unique_ptr_test {

int Target::destructor_count = 0;
int Derived::derived_destructor_count = 0;

Target::~Target() { destructor_count++; }

int Target::get_destructor_count() { return destructor_count; }

std::unique_ptr<Target> create_target() { return std::make_unique<Target>(); }

Derived::~Derived() { derived_destructor_count++; }

int Derived::get_derived_destructor_count() { return derived_destructor_count; }

std::unique_ptr<Base> create_virtual_base() {
  return std::make_unique<Derived>();
}

}  // namespace unique_ptr_test
