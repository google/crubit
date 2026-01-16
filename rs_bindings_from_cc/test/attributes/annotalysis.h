// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ATTRIBUTES_ANNOTALYSIS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ATTRIBUTES_ANNOTALYSIS_H_

#include "absl/base/thread_annotations.h"
#include "support/annotations.h"

// There are too many thread-safety annotations to bother testing,
// but this illustrates the core use case: Crubit should produce (callable)
// bindings for a set of functions with thread-safety annotations, even if
// there is no safe way to call them from Rust (as it doesn't have them).
struct CRUBIT_MUST_BIND ABSL_LOCKABLE MyMutex {};

class ExampleClass {
 public:
  CRUBIT_MUST_BIND
  static int return_42() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) { return 42; }
  CRUBIT_MUST_BIND
  static MyMutex& mu() ABSL_LOCK_RETURNED(mu_) { return mu_; }

 private:
  static MyMutex mu_;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ATTRIBUTES_ANNOTALYSIS_H_
