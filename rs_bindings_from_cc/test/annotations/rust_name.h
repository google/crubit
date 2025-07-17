// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_RUST_NAME_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_RUST_NAME_H_

#include "support/annotations.h"

namespace crubit::test {

CRUBIT_RUST_NAME("free_fn_new_name")
inline void FreeFnOldName() {}

struct CRUBIT_RUST_NAME("StructNewName") StructOldName {};

struct SomeStruct {
  SomeStruct() : field_old_name(24601) {}
  CRUBIT_RUST_NAME("ConstructorNewName")
  SomeStruct(int a, int b, int c) : field_old_name(a + b + c) {}
  CRUBIT_RUST_NAME("MethodNewName") void MethodOldName() const {}
  CRUBIT_RUST_NAME("field_new_name") int field_old_name;
};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_RUST_NAME_H_
