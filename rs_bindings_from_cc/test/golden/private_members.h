// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_MEMBERS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_MEMBERS_H_

#pragma clang lifetime_elision

class SomeClass final {
 public:
  void public_method();
  static void public_static_method();
  int public_member_variable_;

 private:
  void private_method();
  static void private_static_method();
  int private_member_variable_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_MEMBERS_H_
