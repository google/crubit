// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_ACCESS_RULE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_ACCESS_RULE_H_

#pragma clang lifetime_elision

template <typename T>
class A final {};

class B final {
 private:
  struct PrivateMember;

 public:
  A<PrivateMember> a_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TYPE_ALIAS_TYPE_ALIAS_ACCESS_RULE_H_
