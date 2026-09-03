// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_SHARED_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_SHARED_PTR_TEST_HELPERS_H_

#include <cstddef>
#include <memory>

#include "support/annotations.h"

namespace shared_ptr_test {

// Returns the size of the layout of a shared_ptr<int>.
CRUBIT_MUST_BIND inline size_t get_shared_ptr_size() {
  return sizeof(std::shared_ptr<int>);
}

// Returns the alignment of the layout of a shared_ptr<int>.
CRUBIT_MUST_BIND inline size_t get_shared_ptr_alignment() {
  return alignof(std::shared_ptr<int>);
}

// Returns a new shared_ptr<int>.
CRUBIT_MUST_BIND inline std::shared_ptr<int> create_shared_ptr() {
  return std::make_shared<int>(1);
}

// Returns a new shared_ptr<const int>.
CRUBIT_MUST_BIND inline std::shared_ptr<const int> create_shared_ptr_const() {
  return std::make_shared<const int>(1);
}

// Consumes a shared_ptr<int>.
CRUBIT_MUST_BIND inline void destroy_shared_ptr(std::shared_ptr<int>) {}

// Consumes a shared_ptr<const int>.
CRUBIT_MUST_BIND inline void destroy_shared_ptr_const(
    std::shared_ptr<const int>) {}

// Returns an empty shared_ptr<void>.
CRUBIT_MUST_BIND inline std::shared_ptr<void> create_shared_ptr_void_ptr() {
  return std::shared_ptr<void>(nullptr);
}

// Since shared_ptr uses a control block for type erasure, we can get a pointer
// to the base class and the control block will correctly destroy the derived
// type.
struct Base {
  virtual ~Base() = default;
  virtual bool is_derived() const { return false; }
};
struct Derived : public Base {
  explicit Derived(int* count) : count(count) {}
  ~Derived() override {
    if (count != nullptr) {
      (*count)++;
    }
  }
  bool is_derived() const override { return true; }
  int* count = nullptr;
};

CRUBIT_MUST_BIND inline std::shared_ptr<Base> create_virtual_base(int* count) {
  return std::make_shared<Derived>(count);
}

CRUBIT_MUST_BIND inline std::unique_ptr<int> create_unique_ptr() {
  return std::make_unique<int>(42);
}

CRUBIT_MUST_BIND inline std::unique_ptr<int> create_null_unique_ptr() {
  return nullptr;
}

CRUBIT_MUST_BIND inline std::unique_ptr<Base> create_virtual_unique_base(
    int* count) {
  return std::make_unique<Derived>(count);
}

CRUBIT_MUST_BIND inline std::unique_ptr<Base>
create_null_virtual_unique_base() {
  return nullptr;
}

}  // namespace shared_ptr_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_SHARED_PTR_TEST_HELPERS_H_
