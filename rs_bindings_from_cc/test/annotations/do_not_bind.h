// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_DO_NOT_BIND_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_DO_NOT_BIND_H_

#include "support/annotations.h"

namespace crubit::test {

struct ArgumentToBoundOverload {};
struct ArgumentToUnboundOverload {};

CRUBIT_DO_NOT_BIND inline void DoNotBindFn(ArgumentToUnboundOverload) {}
inline void DoNotBindFn(ArgumentToBoundOverload) {}

CRUBIT_DO_NOT_BIND inline void NonAllowListedDoNotBindFn() {}

struct CRUBIT_DO_NOT_BIND DoNotBindStruct {
  struct StructNestedInsideDoNotBindStruct {};
  DoNotBindStruct() = default;
  void MethodOfDoNotBindStruct() {}
};

inline void FunctionWithDoNotBindArgument(DoNotBindStruct) {}

struct StructWithDoNotBindConstructor {
  CRUBIT_DO_NOT_BIND explicit StructWithDoNotBindConstructor(
      ArgumentToUnboundOverload) {}
  explicit StructWithDoNotBindConstructor(ArgumentToBoundOverload) {}
};

struct StructWithDoNotBindMethod {
  CRUBIT_DO_NOT_BIND void DoNotBindMethod(ArgumentToUnboundOverload) {}
  void DoNotBindMethod(ArgumentToBoundOverload) {}
};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_DO_NOT_BIND_H_
