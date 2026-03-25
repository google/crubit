// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_SHARED_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_SHARED_PTR_TEST_HELPERS_H_

#include <memory>

#include "support/annotations.h"

namespace shared_ptr_test {

struct TwoWords {
  void* ptr1;
  void* ptr2;
};

CRUBIT_MUST_BIND inline size_t get_shared_ptr_size() {
  return sizeof(std::shared_ptr<const int>);
}

CRUBIT_MUST_BIND inline size_t get_shared_ptr_alignment() {
  return alignof(std::shared_ptr<const int>);
}

CRUBIT_MUST_BIND inline std::shared_ptr<const int> create_shared_ptr() {
  return std::make_shared<int>(1);
}
CRUBIT_MUST_BIND inline void destroy_shared_ptr(std::shared_ptr<const int>) {}
CRUBIT_MUST_BIND inline std::shared_ptr<const char> create_shared_ptr_char() {
  return std::make_shared<char>('a');
}
CRUBIT_MUST_BIND inline std::shared_ptr<const short> create_shared_ptr_short() {
  return std::make_shared<short>(static_cast<short>(1));
}
CRUBIT_MUST_BIND inline std::shared_ptr<const void>
create_shared_ptr_void_ptr() {
  return std::shared_ptr<const void>(nullptr);
}
CRUBIT_MUST_BIND inline std::shared_ptr<const TwoWords>
create_shared_ptr_two_words() {
  return std::make_shared<TwoWords>();
}

// Since shared_ptr uses a control block for type erasure, we can get a pointer
// to the base class and the control block will correctly destroy the derived
// type.
struct Base {
  virtual ~Base() = default;
  static inline int derived_destructor_count = 0;
  virtual bool is_derived() const { return false; }
};
struct Derived : public Base {
  ~Derived() override { derived_destructor_count++; }
  bool is_derived() const override { return true; }
};

CRUBIT_MUST_BIND inline std::shared_ptr<const Base> create_virtual_base() {
  return std::make_shared<Derived>();
}

CRUBIT_MUST_BIND inline int get_derived_destructor_count() {
  return Base::derived_destructor_count;
}

// Using a custom deleter via shared_ptr constructors.
struct CustomDelete {
  static inline int custom_delete_count = 0;
};
struct CustomDeleter {
  void operator()(const CustomDelete* p) const {
    CustomDelete::custom_delete_count++;
    delete p;
  }
};

CRUBIT_MUST_BIND inline std::shared_ptr<const CustomDelete>
create_custom_delete() {
  return std::shared_ptr<const CustomDelete>(new CustomDelete(),
                                             CustomDeleter());
}

CRUBIT_MUST_BIND inline int get_custom_delete_count() {
  return CustomDelete::custom_delete_count;
}

}  // namespace shared_ptr_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_SHARED_PTR_TEST_HELPERS_H_
