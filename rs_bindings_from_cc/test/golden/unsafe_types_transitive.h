// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSAFE_TYPES_TRANSITIVE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSAFE_TYPES_TRANSITIVE_H_

struct PublicPointer {
  int* p;
};

class PrivatePointer {
  friend int DerefPrivatePointer(PrivatePointer p);

  int* p_;
};

struct TransitivePublicPointer {
  PublicPointer pub;
  PrivatePointer priv;
};

union Union {
  int i;
  float f;
};

int DerefPointer(int* p);

int DerefPublicPointer(PublicPointer p);

int DerefPrivatePointer(PrivatePointer p);

int DerefTransitivePublicPointer(TransitivePublicPointer p);

int ReadUnion(Union u);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSAFE_TYPES_TRANSITIVE_H_
