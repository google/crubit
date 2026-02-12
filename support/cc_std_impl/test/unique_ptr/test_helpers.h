// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_UNIQUE_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_UNIQUE_PTR_TEST_HELPERS_H_

#include <memory>

#include "support/annotations.h"

namespace unique_ptr_test {

CRUBIT_MUST_BIND inline std::unique_ptr<int> create_unique_ptr() {
  return std::make_unique<int>(1);
}
CRUBIT_MUST_BIND inline void destroy_unique_ptr(std::unique_ptr<int>) {}

struct Base {
  virtual ~Base() = default;
  static inline int derived_destructor_count = 0;
  virtual bool is_derived() const { return false; }
};
struct Derived : public Base {
  ~Derived() override { derived_destructor_count++; }
  bool is_derived() const override { return true; }
};

CRUBIT_MUST_BIND inline std::unique_ptr<Base> create_virtual_base() {
  return std::make_unique<Derived>();
}

CRUBIT_MUST_BIND inline int get_derived_destructor_count() {
  return Base::derived_destructor_count;
}

struct CustomDelete {
  static void operator delete(void* p) {
    custom_delete_count++;
    ::operator delete(p);
  }
  static inline int custom_delete_count = 0;
};

CRUBIT_MUST_BIND inline std::unique_ptr<CustomDelete> create_custom_delete() {
  return std::make_unique<CustomDelete>();
}

CRUBIT_MUST_BIND inline int get_custom_delete_count() {
  return CustomDelete::custom_delete_count;
}

}  // namespace unique_ptr_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_UNIQUE_PTR_TEST_HELPERS_H_
