// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_SFINAE_ATOMIC_REGRESSION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_SFINAE_ATOMIC_REGRESSION_H_

#include <atomic>
#include <cstdint>

struct HiddenPointer {
  explicit HiddenPointer(void* p) : hidden_(reinterpret_cast<uintptr_t>(p)) {}

 private:
  uintptr_t hidden_;
};

struct Slot {
  struct Data {
    HiddenPointer ptr{nullptr};
  };

  std::atomic<Data> data{Data()};
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_SFINAE_ATOMIC_REGRESSION_H_
