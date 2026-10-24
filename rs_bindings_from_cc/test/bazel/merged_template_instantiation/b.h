// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_B_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_B_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/bazel/merged_template_instantiation/template.h"

namespace b_ns {

// Implicitly instantiates `Box<int16_t>`, and declares its implicit copy
// constructor, in the precompiled module of `:b`.
inline template_ns::Box<int16_t> Copy(const template_ns::Box<int16_t>& box) {
  return box;
}

// Implicitly instantiates `Box<int>`, and calls its `GetOne` method, but
// doesn't declare its implicit copy constructor, in the precompiled module of
// `:b`.
inline int GetOne(const template_ns::Box<int>& box) { return box.GetOne(); }

}  // namespace b_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_B_H_
