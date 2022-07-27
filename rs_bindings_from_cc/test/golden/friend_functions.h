// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_FRIEND_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_FRIEND_FUNCTIONS_H_

#pragma clang lifetime_elision

class SomeClass final {
 public:
  // Friend functions that are visible via ADL.
  friend void visible_val(SomeClass);
  friend void visible_ref(SomeClass&);
  friend void visible_cref(const SomeClass&);
  friend void visible_rref(SomeClass&&);

  // Friend functions that are not visible via ADL and thus generate no
  // top-level bindings.
  friend void invisible_int(int param);
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_FRIEND_FUNCTIONS_H_
