// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/fn_ref.h"

#include <cstdint>
#include <type_traits>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using ::testing::Eq;

static_assert(std::is_trivially_copyable_v<rs::FnRef<int(int)>>);
static_assert(std::is_trivially_copyable_v<rs::FnRef<int(int) const>>);
static_assert(sizeof(rs::FnRef<int(int)>) == 2 * sizeof(void*));
static_assert(sizeof(rs::internal::FnRefPayload) == 2 * sizeof(void*));

int AddOne(int x) { return x + 1; }

TEST(FnRefTest, CallConstLambda) {
  auto lambda = [](int x) { return x * 2; };
  rs::FnRef<int(int) const> ref(lambda);
  EXPECT_THAT(ref(21), Eq(42));
}

TEST(FnRefTest, CallMutableLambda) {
  int count = 0;
  auto lambda = [&count](int x) mutable {
    count += x;
    return count;
  };
  rs::FnRef<int(int)> ref(lambda);
  EXPECT_THAT(ref(10), Eq(10));
  EXPECT_THAT(ref(5), Eq(15));
  EXPECT_THAT(count, Eq(15));
}

TEST(FnRefTest, CallFunctionPointer) {
  rs::FnRef<int(int) const> ref(AddOne);
  EXPECT_THAT(ref(41), Eq(42));
}

TEST(FnRefTest, VoidReturnType) {
  int called = 0;
  auto lambda = [&called]() { called++; };
  rs::FnRef<void()> ref(lambda);
  ref();
  EXPECT_THAT(called, Eq(1));
}

TEST(FnRefTest, PayloadInvoker) {
  auto lambda = [](int a, int b) { return a + b; };
  rs::FnRef<int(int, int) const> ref(lambda);
  auto payload = ref.payload();
  auto invoker =
      reinterpret_cast<int (*)(const void*, int, int)>(payload.invoker);
  EXPECT_THAT(invoker(payload.data, 20, 22), Eq(42));
}

}  // namespace
