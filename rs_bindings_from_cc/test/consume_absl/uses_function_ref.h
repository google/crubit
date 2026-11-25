// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_FUNCTION_REF_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_FUNCTION_REF_H_

#include "absl/functional/function_ref.h"

// Calls the function ref and returns void.
void CallVoidVoid(absl::FunctionRef<void()> f);

// Calls an int mapping function ref multiple times (summing a range from 0 to
// n).
int CallIntIntMultiple(absl::FunctionRef<int(int) const> f, int n);

// A move-only layout-compatible parameter passed by value.
struct NonTrivial {
  explicit NonTrivial(int v) : value(v) {}
  NonTrivial(const NonTrivial&) = delete;
  NonTrivial(NonTrivial&&) = default;
  int value;
};
NonTrivial CreateNonTrivial(int v);
int CallWithNonTrivial(absl::FunctionRef<int(NonTrivial)> f, NonTrivial val);

// A C++ using alias of FunctionRef.
using FunctionRefAlias = absl::FunctionRef<int(int)>;
int CallAlias(FunctionRefAlias f, int i);

// Invokes `f` concurrently from two `std::thread`s and joins both before
// returning.
void CallFromTwoThreads(absl::FunctionRef<void()> f);

// Constructors taking `absl::FunctionRef` for both `Unpin` (`From`) and
// `!Unpin` (`CtorNew`).
struct CallbackRunner {
  explicit CallbackRunner(absl::FunctionRef<void()> f);
  int calls = 0;
};

struct PinnedCallbackRunner {
  explicit PinnedCallbackRunner(absl::FunctionRef<void()> f);
  PinnedCallbackRunner(const PinnedCallbackRunner&) = delete;
  int calls = 0;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_FUNCTION_REF_H_
