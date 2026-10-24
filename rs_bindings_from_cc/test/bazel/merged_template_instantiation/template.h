// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_TEMPLATE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_TEMPLATE_H_

#include "support/annotations.h"

namespace template_ns {

// Trivially copyable, so instantiations of it are Rust-movable.
//
// `CRUBIT_ALWAYS_INSTANTIATE` makes Crubit bind all methods of instantiations
// (rather than just special members).
template <typename T>
struct CRUBIT_ALWAYS_INSTANTIATE Box final {
  int GetOne() const { return 1; }
  T value;
};

}  // namespace template_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_TEMPLATE_H_
