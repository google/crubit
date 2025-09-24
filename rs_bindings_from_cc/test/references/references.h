// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_REFERENCES_REFERENCES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_REFERENCES_REFERENCES_H_

class TypeWithPtrConstructor {
 public:
  explicit TypeWithPtrConstructor(int* ptr) {}
};

class TypeWithNonNullPtrConstructor {
 public:
  explicit TypeWithNonNullPtrConstructor(int* _Nonnull ptr) {}
};

class TypeWithReferenceConstructor {
 public:
  explicit TypeWithReferenceConstructor(int& ref) {}
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_REFERENCES_REFERENCES_H_
