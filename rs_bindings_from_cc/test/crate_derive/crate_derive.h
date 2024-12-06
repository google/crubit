// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRATE_DERIVE_CRATE_DERIVE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRATE_DERIVE_CRATE_DERIVE_H_

struct [[clang::annotate("crubit_internal_trait_derive", "!Clone",
                         "Debug")]] StructWithDerives {
  int x;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CRATE_DERIVE_CRATE_DERIVE_H_
