// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_VIEW_OWNER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_VIEW_OWNER_H_

#include "support/annotations.h"

namespace crubit::test {

// CRUBIT_VIEW and CRUBIT_OWNER do not yet change the bindings (see
// annotations.h). This golden records today's behaviour; `view_owner_unsafe`
// shows the one effect, under the `unsafe_view` feature.

// The pointers are private, so that a public pointer field does not make the
// records unsafe on its own.

// A view: borrows what `data_` points to.
class CRUBIT_VIEW View final {
 public:
  int Get() const { return *data_; }

 private:
  const int* data_ = nullptr;
};

// An owner: owns what `data_` points to.
class CRUBIT_OWNER Owner final {
 public:
  Owner() = default;
  Owner(const Owner&) = delete;
  Owner& operator=(const Owner&) = delete;
  ~Owner() { delete data_; }

 private:
  int* data_ = nullptr;
};

// Neither annotation, for comparison.
class Plain final {
 public:
  int Get() const { return *data_; }

 private:
  const int* data_ = nullptr;
};

inline int ReadView(View v) { return v.Get(); }
inline int ReadPlain(Plain p) { return p.Get(); }

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_VIEW_OWNER_H_
