// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DUNDER_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DUNDER_TYPES_H_

struct __DunderType {
  int x;
};

typedef struct {
  int x;
} __DunderAnonTypedef;

typedef struct __DunderTypedef {
  int x;
} __DunderTypedef;

typedef __DunderType AliasToDunderType;
typedef __DunderTypedef AliasToDunderTypedef;
typedef __DunderAnonTypedef AliasToDunderAnonTypedef;

typedef AliasToDunderType AliasToAliasToDunderType;
typedef AliasToDunderTypedef AliasToAliasToDunderTypedef;
typedef AliasToDunderAnonTypedef AliasToAliasToDunderAnonTypedef;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DUNDER_TYPES_H_
