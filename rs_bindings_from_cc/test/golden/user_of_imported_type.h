// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_IMPORTED_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_IMPORTED_TYPE_H_

#include "rs_bindings_from_cc/test/golden/trivial_type.h"

#pragma clang lifetime_elision

test_namespace_bindings::Trivial UsesImportedType(
    test_namespace_bindings::Trivial t);

struct UserOfImportedType final {
  test_namespace_bindings::Trivial* trivial;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_IMPORTED_TYPE_H_
