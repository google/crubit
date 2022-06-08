// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

#pragma clang lifetime_elision

struct TrivialCustomType final {
  // Replace this with some other unsupported operator, if support is later
  // added.
  bool operator||(const TrivialCustomType&) const;

  int i;
};

struct NontrivialCustomType final {
  NontrivialCustomType(NontrivialCustomType&&);
  // Replace this with some other unsupported operator, if support is later
  // added.
  bool operator||(const NontrivialCustomType&) const;

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

  // Doc comment for an unsupported field.
  NestedStruct nested_struct;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_
