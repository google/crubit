// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_

#include "rs_bindings_from_cc/test/golden/inheritance.h"

#pragma clang lifetime_elision

// The same as Derived from inheritance.h, but in a different build target.
//
// This tests inheritance across library boundaries.
//
// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
struct Derived2 final : virtual Base0, Base1, Base2 {
  char derived_1;
};

class VirtualDerived2 : public virtual VirtualBase1,
                        public virtual VirtualBase2 {};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_
