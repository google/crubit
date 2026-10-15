// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MODULE_ORDER_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MODULE_ORDER_A_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/bazel/module_order/template.h"
#include "support/annotations.h"

namespace a_ns {

// Implicitly instantiates `Box<int>`, and declares its implicit copy
// constructor, in the precompiled module of `:a`.
inline template_ns::Box<int> Copy(const template_ns::Box<int>& box) {
  return box;
}

// Implicitly instantiates `Box<int16_t>`, and calls its `GetOne` method, but
// doesn't declare its implicit copy constructor, in the precompiled module of
// `:a`.
inline int GetOne(const template_ns::Box<int16_t>& box) { return box.GetOne(); }

// A template defined only in the precompiled module of `:a`. Its
// instantiations in `lib.h` are ordered relative to those of `b_ns::BBox` by
// the positions of the two templates, which are in two different modules.
template <typename T>
struct CRUBIT_ALWAYS_INSTANTIATE ABox final {
  T value;
};

}  // namespace a_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MODULE_ORDER_A_H_
