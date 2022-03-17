// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

#pragma clang lifetime_elision

struct NontrivialCustomType final {
  NontrivialCustomType(NontrivialCustomType&&);

  int i;
};

void UnsupportedParamType(NontrivialCustomType n);
NontrivialCustomType UnsupportedReturnType();

NontrivialCustomType MultipleReasons(NontrivialCustomType n, int,
                                     NontrivialCustomType n2);

namespace ns {
void FunctionInNamespace();
struct StructInNamespace final {
  void NonStaticMemberFunction();
  void StaticMemberFunction();
};
}  // namespace ns

struct ContainingStruct final {
  struct NestedStruct final {
    void NonStaticMemberFunction();
    void StaticMemberFunction();
  };
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_
