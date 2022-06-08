// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_
#include <stdint.h>

#pragma clang lifetime_elision

// Using classes to force these to be non-POD.
// In the Itanium ABI, the tail padding of POD types cannot be reused by other
// objects, even if the POD type is potentially-overlapping.

class Base0 {};
class Base1 {
  int64_t b1_1_;  // NOLINT(clang-diagnostic-unused-private-field)
  char b1_2_;     // NOLINT(clang-diagnostic-unused-private-field)
};

class Base2 {
  int16_t b2_1_;  // NOLINT(clang-diagnostic-unused-private-field)
};

struct Derived final : Base0, Base1, Base2 {
  char derived_1;
};

class VirtualBase1 : public virtual Base1 {};
class VirtualBase2 : public virtual Base1 {};
class VirtualDerived : public virtual VirtualBase1,
                       public virtual VirtualBase2 {};

class MyAbstractClass {
  virtual void PureVirtualMethod() = 0;
  virtual ~MyAbstractClass();
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_
