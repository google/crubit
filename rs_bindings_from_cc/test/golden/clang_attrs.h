// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_

struct alignas(64) HasCustomAlignment {};

struct HasFieldWithCustomAlignment {
  HasCustomAlignment field;
};

struct InheritsFromBaseWithCustomAlignment : public HasCustomAlignment {};

struct HasCustomAlignmentWithGnuAttr {
} __attribute__((aligned(64)));

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_
