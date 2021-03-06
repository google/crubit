// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_

#pragma clang lifetime_elision

class PolymorphicBase {
 public:
  virtual ~PolymorphicBase();
};
class PolymorphicBase2 {
 public:
  virtual void Foo();
  virtual ~PolymorphicBase2();
};

class PolymorphicDerived : PolymorphicBase, PolymorphicBase2 {};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_
