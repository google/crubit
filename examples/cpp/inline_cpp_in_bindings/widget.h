// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_INLINE_CPP_IN_BINDINGS_WIDGET_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_INLINE_CPP_IN_BINDINGS_WIDGET_H_

namespace widget {

// A plain struct. `widget_custom.rs` uses its generated Rust bindings.
struct Widget {
  int size;
};

// A function template. Crubit can't bind templates directly.
template <typename T>
T Twice(T x) {
  return x + x;
}

}  // namespace widget

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_INLINE_CPP_IN_BINDINGS_WIDGET_H_
