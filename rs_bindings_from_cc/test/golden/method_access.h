// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_ACCESS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_ACCESS_H_

struct Struct {
  void AccessNone();

 public:
  void AccessPublic();

 protected:
  void AccessProtected();

 private:
  void AccessPrivate();
};

class Class {
  void AccessNone();

 public:
  void AccessPublic();

 protected:
  void AccessProtected();

 private:
  void AccessPrivate();
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_METHOD_ACCESS_H_
