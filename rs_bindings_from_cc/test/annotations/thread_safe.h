// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_THREAD_SAFE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_THREAD_SAFE_H_

#include "support/annotations.h"

namespace crubit::test {

// A simple thread-safe struct.
class CRUBIT_THREAD_SAFE ThreadSafeStruct final {
 public:
  ThreadSafeStruct() = default;
  ThreadSafeStruct(const ThreadSafeStruct&) = delete;
  ThreadSafeStruct& operator=(const ThreadSafeStruct&) = delete;

  int ConstGet() const { return x_; }
  // A non-const method for testing the generation behavior.
  // The implementation doesn't actually do anything non-const, but it doesn't
  // matter for what we are testing, here.
  int NonConstGet() { return x_; }

 private:
  int x_ = 0;
};

// A regular (non-thread-safe) struct for comparison.
class RegularStruct final {
 public:
  int ConstGet() const { return x_; }
  int NonConstGet() { return x_; }

 private:
  int x_ = 0;
};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_THREAD_SAFE_H_
