// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/callables/std_function_ref.h"

#include <thread>  // NOLINT(build/c++11)

void CallStdFunctionRefVoidVoid(std::function_ref<void()> f) { f(); }
int CallStdFunctionRefIntInt(std::function_ref<int(int) const> f, int n) {
  return f(n);
}
void CallStdFunctionRefNoexcept(std::function_ref<void() noexcept> f) { f(); }
int CallStdFunctionRefMultipleTimes(std::function_ref<int(int)> f) {
  return f(f(1));
}
void CallStdFunctionRefFromTwoThreads(std::function_ref<void()> f) {
  std::thread t1([f] { f(); });
  std::thread t2([f] { f(); });
  t1.join();
  t2.join();
}
