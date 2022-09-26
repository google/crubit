// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_METHOD_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_METHOD_H_

template <class T>
class Ptr {
 public:
  Ptr(T* ptr) : ptr_(ptr) {}

 private:
  T* ptr_;
};

class Outer {
 private:
  class Inner {
   private:
    int v_ = 1;  // NOLINT: unused variable necessary to trigger the test case.
  };
  void ShouldNotBeImportedSinceItIsPrivate(Ptr<Inner> ptr) {}
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_METHOD_H_
