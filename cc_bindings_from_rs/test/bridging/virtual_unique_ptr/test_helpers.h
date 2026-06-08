// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_VIRTUAL_UNIQUE_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_VIRTUAL_UNIQUE_PTR_TEST_HELPERS_H_

#include <memory>

#include "support/annotations.h"

namespace virtual_unique_ptr_test {

inline int derived_destructor_count = 0;

struct Base {
  virtual ~Base() = default;
};

struct Derived : public Base {
  ~Derived() override { derived_destructor_count++; }
};

CRUBIT_MUST_BIND inline std::unique_ptr<Base> create_virtual_base() {
  return std::make_unique<Derived>();
}

CRUBIT_MUST_BIND inline int get_derived_destructor_count() {
  return derived_destructor_count;
}

}  // namespace virtual_unique_ptr_test

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_VIRTUAL_UNIQUE_PTR_TEST_HELPERS_H_
