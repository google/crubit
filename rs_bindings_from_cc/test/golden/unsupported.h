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

  // clang::DeclarationName::NameKind::CXXConversionFunctionName
  explicit operator int() const { return i; }

  int i;
};

struct NontrivialCustomType final {
  NontrivialCustomType(NontrivialCustomType&&);
  // Replace this with some other unsupported operator, if support is later
  // added.
  bool operator||(const NontrivialCustomType&) const;

  int i;
};

// Structs with packed layout are unsupported.
struct PackedLayout final {
  char char_var;
  __attribute__((packed)) int int_var;
};

// This function can't have bindings because of both its return type and its
// parameter type.
//
// TODO(jeanpierreda): Use a dedicated unsupported type or attribute so that this
// test is more stable and doesn't depend on which exact types/features are not
// currently supported.
volatile int* MultipleReasons(volatile int* n);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_
