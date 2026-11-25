// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/consume_absl/uses_function_ref.h"

#include <thread>  // NOLINT(build/c++11)
#include <utility>

void CallVoidVoid(absl::FunctionRef<void()> f) { f(); }

int CallIntIntMultiple(absl::FunctionRef<int(int) const> f, int n) {
  int sum = 0;
  for (int i = 0; i < n; ++i) {
    sum += f(i);
  }
  return sum;
}

NonTrivial CreateNonTrivial(int v) { return NonTrivial(v); }

int CallWithNonTrivial(absl::FunctionRef<int(NonTrivial)> f, NonTrivial val) {
  return f(std::move(val));
}

int CallAlias(FunctionRefAlias f, int i) { return f(i); }

void CallFromTwoThreads(absl::FunctionRef<void()> f) {
  std::thread t1([f] { f(); });
  std::thread t2([f] { f(); });
  t1.join();
  t2.join();
}

CallbackRunner::CallbackRunner(absl::FunctionRef<void()> f) {
  f();
  calls = 1;
}

PinnedCallbackRunner::PinnedCallbackRunner(absl::FunctionRef<void()> f) {
  f();
  calls = 1;
}
