// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CALLABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CALLABLES_H_

#include "support/rs_std/dyn_callable.h"

int apply(rs_std::DynCallable<int(int) const> callback, int arg);

int apply_mut(rs_std::DynCallable<int(int)> callback, int arg);

int apply_once(rs_std::DynCallable<int(int) &&> callback, int arg);

class NotCABICompatible {
 public:
  explicit NotCABICompatible(int x) : private_(x) {}

  int get() const { return private_; }

 private:
  int private_;
};

void rust_inspect_non_c_abi_compatible_struct(
    rs_std::DynCallable<NotCABICompatible(NotCABICompatible)> cb);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CALLABLES_H_
