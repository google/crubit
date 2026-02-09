// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_UNIQUE_PTR_UNIQUE_PTR_DYN_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_UNIQUE_PTR_UNIQUE_PTR_DYN_TEST_HELPERS_H_

#include <memory>

#include "support/annotations.h"

namespace unique_ptr_dyn_test {

struct BaseWithVirtualDestructor {
  virtual ~BaseWithVirtualDestructor();
  static int instances();
  static void set_instances(int n);
};

struct Derived : public BaseWithVirtualDestructor {
  ~Derived() override;
};

struct WithOverloadedDelete {
  static bool delete_called();
  static void set_delete_called(bool b);
  void operator delete(void* p);
};

CRUBIT_MUST_BIND inline std::unique_ptr<BaseWithVirtualDestructor>
create_base() {
  return std::make_unique<Derived>();
}

CRUBIT_MUST_BIND inline std::unique_ptr<Derived> create_derived() {
  return std::make_unique<Derived>();
}

CRUBIT_MUST_BIND inline std::unique_ptr<WithOverloadedDelete>
create_with_overloaded_delete() {
  return std::unique_ptr<WithOverloadedDelete>(new WithOverloadedDelete());
}

}  // namespace unique_ptr_dyn_test

#endif
