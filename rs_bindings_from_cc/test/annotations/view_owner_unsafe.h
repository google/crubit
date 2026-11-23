// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_VIEW_OWNER_UNSAFE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_VIEW_OWNER_UNSAFE_H_

#include "support/annotations.h"

namespace crubit::test {

// Built with the `unsafe_view` feature: the CRUBIT_VIEW record is unsafe, the
// CRUBIT_OWNER record is not.

class CRUBIT_VIEW UnsafeView final {
 public:
  int Get() const { return *data_; }

 private:
  const int* data_ = nullptr;
};

class CRUBIT_OWNER SafeOwner final {
 public:
  SafeOwner() = default;
  SafeOwner(const SafeOwner&) = delete;
  SafeOwner& operator=(const SafeOwner&) = delete;
  ~SafeOwner() { delete data_; }

 private:
  int* data_ = nullptr;
};

inline int ReadUnsafeView(UnsafeView v) { return v.Get(); }

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_VIEW_OWNER_UNSAFE_H_
