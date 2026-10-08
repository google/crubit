// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_UNIQUE_PTR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_UNIQUE_PTR_TEST_HELPERS_H_

#include <cstdint>
#include <memory>

#include "support/annotations.h"

namespace unique_ptr_test {

// The test objects below each hold a copy of a `token` while alive. Tests check
// `token.use_count()` to tell whether an object has been destroyed, without any
// global state.

// For unique_ptr test
struct Target {
  explicit Target(std::shared_ptr<int32_t> token);

 private:
  std::shared_ptr<int32_t> token;
};

CRUBIT_MUST_BIND std::unique_ptr<Target> create_target(
    std::shared_ptr<int32_t> token);

// For virtual_unique_ptr test
struct Base {
  virtual ~Base() = default;
};

struct Derived : public Base {
  explicit Derived(std::shared_ptr<int32_t> token);

 private:
  std::shared_ptr<int32_t> token;
};

CRUBIT_MUST_BIND std::unique_ptr<Base> create_virtual_base(
    std::shared_ptr<int32_t> token);

}  // namespace unique_ptr_test

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_UNIQUE_PTR_TEST_HELPERS_H_
