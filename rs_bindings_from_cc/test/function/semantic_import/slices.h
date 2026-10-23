// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SLICES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SLICES_H_

// This is kept apart from `semantic_import.h` because including `slice_ref.h`
// brings in bindings for many standard library types, which would swamp the
// golden files for `semantic_import.h`.

#include "support/rs_std/slice_ref.h"

// `rs_std::SliceRef` maps to a Rust slice pointer. Crubit relies on the two
// having the same layout, so the getter reads the field directly, but the
// setter keeps using a thunk: only primitives and plain pointers get a
// thunkless setter.
class Slices {
 public:
  rs_std::SliceRef<const int> s() const { return s_; }
  void set_s(rs_std::SliceRef<const int> s) { s_ = s; }

 private:
  rs_std::SliceRef<const int> s_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SLICES_H_
