// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/fn.h"

#include <cstdint>
#include <memory>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "support/movable.h"

namespace {

using ::testing::Eq;

TEST(FnTest, CallConstLambda) {
  rs::Fn<int(int) const> inv([](int x) { return x * 3; });
  EXPECT_THAT(inv(14), Eq(42));
}

TEST(FnTest, CallMutableLambda) {
  int count = 10;
  rs::Fn<int(int)> inv([count](int x) mutable {
    count += x;
    return count;
  });
  EXPECT_THAT(inv(5), Eq(15));
  EXPECT_THAT(inv(5), Eq(20));
}

TEST(FnTest, CallMoveOnlyLambda) {
  auto ptr = std::make_unique<int>(42);
  rs::Fn<int() &&> inv([p = std::move(ptr)]() { return *p; });
  EXPECT_THAT(std::move(inv)(), Eq(42));
}

TEST(FnTest, DestructorCleansUp) {
  auto tracker = std::make_shared<int>(100);
  EXPECT_THAT(tracker.use_count(), Eq(1));
  {
    rs::Fn<void() const> inv([tracker]() {});
    EXPECT_THAT(tracker.use_count(), Eq(2));
  }
  EXPECT_THAT(tracker.use_count(), Eq(1));
}

TEST(FnTest, MoveConstructor) {
  rs::Fn<int(int) const> inv1([](int x) { return x + 10; });
  rs::Fn<int(int) const> inv2(std::move(inv1));
  EXPECT_FALSE(inv1);
  EXPECT_TRUE(inv2);
  EXPECT_THAT(inv2(32), Eq(42));
}

TEST(FnTest, ReleasePayload) {
  auto tracker = std::make_shared<int>(200);
  rs::internal::FnPayload payload;
  {
    rs::Fn<int(int) const> inv([tracker](int x) { return *tracker + x; });
    EXPECT_THAT(tracker.use_count(), Eq(2));
    payload = std::move(inv).release_payload();
    EXPECT_FALSE(inv);
  }
  // Even though `inv` went out of scope, tracker is still alive because payload
  // holds it.
  EXPECT_THAT(tracker.use_count(), Eq(2));

  auto invoker = reinterpret_cast<int (*)(const void*, int)>(payload.invoker);
  EXPECT_THAT(invoker(payload.data, 5), Eq(205));

  // Destroying via payload.destroyer should release tracker.
  payload.destroyer(payload.data);
  EXPECT_THAT(tracker.use_count(), Eq(1));
}

TEST(FnTest, Movable) {
  rs::Movable<rs::Fn<int(int) const>> m1(
      rs::Fn<int(int) const>([](int x) { return x * 2; }));
  rs::Movable<rs::Fn<int(int) const>> m2(std::move(m1));
  EXPECT_FALSE(m1);
  EXPECT_TRUE(m2);
  EXPECT_THAT((*m2)(21), Eq(42));
}

}  // namespace
