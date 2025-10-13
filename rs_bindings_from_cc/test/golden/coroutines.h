// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COROUTINES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COROUTINES_H_

// This is a fake version of c9::Co (http://<internal link>.h), constructed specifically
// for golden tests to avoid generating everything that //util/c9/co generates.
#include "rs_bindings_from_cc/test/c9/co.h"

namespace c9 {

// (Sometimes) change threads, then set the supplied bool and finish.
Co<void> SetBool(bool& b);

// Return 17, sometimes changing threads first.
Co<int> ReturnInt();

}  // namespace c9

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COROUTINES_H_
