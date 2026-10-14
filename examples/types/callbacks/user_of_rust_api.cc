// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <memory>
#include <utility>

#include "gtest/gtest.h"
#include "absl/functional/any_invocable.h"
#include "examples/types/callbacks/rust_api.h"

namespace {

TEST(UserOfRustApiTest, ApplyFnWithLambda) {
  EXPECT_EQ(rust_api::apply_fn([](int32_t x) { return x * 2; }, 21), 42);
}

TEST(UserOfRustApiTest, ApplyFnWithAnyInvocable) {
  absl::AnyInvocable<int32_t(int32_t) const> callback = [](int32_t x) {
    return x * 3;
  };
  EXPECT_EQ(rust_api::apply_fn(callback, 14), 42);
}

TEST(UserOfRustApiTest, ApplyFnMut) {
  int32_t total = 0;
  auto lambda = [&total](int32_t x) mutable {
    total += x;
    return total;
  };
  EXPECT_EQ(rust_api::apply_fn_mut(lambda, 10), 10);
  EXPECT_EQ(rust_api::apply_fn_mut(lambda, 32), 42);
  EXPECT_EQ(total, 42);
}

TEST(UserOfRustApiTest, ApplyFnOnce) {
  auto ptr = std::make_unique<int32_t>(10);
  EXPECT_EQ(rust_api::apply_fn_once(
                [p = std::move(ptr)](int32_t x) { return x + *p; }, 32),
            42);
}

}  // namespace
