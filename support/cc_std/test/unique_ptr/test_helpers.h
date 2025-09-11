// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_TEST_UNIQUE_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_TEST_UNIQUE_PTR_TEST_HELPERS_H_

#include <memory>

#include "support/annotations.h"

namespace unique_ptr_test {

CRUBIT_MUST_BIND inline std::unique_ptr<int> create_unique_ptr() {
  return std::make_unique<int>(1);
}
CRUBIT_MUST_BIND inline void destroy_unique_ptr(std::unique_ptr<int>) {}

}  // namespace unique_ptr_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_TEST_UNIQUE_PTR_TEST_HELPERS_H_
