// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_UNIQUE_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_UNIQUE_PTR_TEST_HELPERS_H_

#include <memory>

#include "support/annotations.h"

namespace unique_ptr_test {

// For unique_ptr test
struct Target {
  ~Target();
  CRUBIT_MUST_BIND static int get_destructor_count();

 private:
  static int destructor_count;
};

CRUBIT_MUST_BIND std::unique_ptr<Target> create_target();

// For virtual_unique_ptr test
struct Base {
  virtual ~Base() = default;
};

struct Derived : public Base {
  ~Derived() override;
  CRUBIT_MUST_BIND static int get_derived_destructor_count();

 private:
  static int derived_destructor_count;
};

CRUBIT_MUST_BIND std::unique_ptr<Base> create_virtual_base();

}  // namespace unique_ptr_test

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_UNIQUE_PTR_TEST_HELPERS_H_
