// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEF_INCOMPLETE_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEF_INCOMPLETE_TYPES_H_

extern "C" {
typedef struct IncompleteExternC IncompleteExternC;
}

typedef struct Incomplete Incomplete;

struct HasPointerToIncompleteTypedefs {
  IncompleteExternC* incomplete_extern_c;
  Incomplete* incomplete;
};

// An elaborated type specifier inside a class declares the struct in the
// nearest enclosing namespace, not in the class (b/576260039).
class OuterWithElaboratedTypedef {
 public:
  typedef const struct IncompleteFromElaboratedTypedef* IncompletePtr;
};

namespace ns {
class Outer {
 public:
  typedef const struct NsIncompleteFromElaboratedTypedef* IncompletePtr;
};
}  // namespace ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEF_INCOMPLETE_TYPES_H_
