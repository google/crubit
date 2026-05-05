// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_CORO_WRAPPER_CORO_WRAPPER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_CORO_WRAPPER_CORO_WRAPPER_H_

#include <coroutine>

#include "support/annotations.h"

namespace coro_test {

// A fake type that simulates a coroutine return type.
struct [[clang::coro_return_type]] FakeFuture {
  struct promise_type {
    FakeFuture get_return_object() { return {}; }
    std::suspend_never initial_suspend() { return {}; }
    std::suspend_never final_suspend() noexcept { return {}; }
    void unhandled_exception() {}
    void return_void() {}
  };
};

CRUBIT_MUST_BIND [[clang::coro_wrapper]] FakeFuture my_coro_wrapper_function() {
  return {};
}

}  // namespace coro_test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_CORO_WRAPPER_CORO_WRAPPER_H_
