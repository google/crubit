// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/async_fn/async_fn.h"

#include <cstdint>
#include <utility>

#include "gtest/gtest.h"
#include "support/internal/slot.h"
#include "support/rs_std/waker.h"

namespace {

class NoopWaker final : public rs_std::Waker {
 public:
  NoopWaker() = default;
  void WakeByRef() override {}
  void WakeAndDestroy() override {}
  rs_std::Waker* Clone() override { return this; }
  void Destroy() override {}
};

TEST(AsyncFnTest, Add) {
  NoopWaker waker;
  crubit::DynErasedFuture<int32_t> fut = async_fn::add(40, 2);
  crubit::Slot<int32_t> out;
  ASSERT_TRUE(fut.Poll(&waker, out.Get()));
  EXPECT_EQ(std::move(out).AssumeInitAndTakeValue(), 42);
}

TEST(AsyncFnTest, ReturnStructWithDrop) {
  NoopWaker waker;
  crubit::DynErasedFuture<async_fn::StructWithDrop> fut =
      async_fn::return_struct_with_drop(123);
  crubit::Slot<async_fn::StructWithDrop> out;
  ASSERT_TRUE(fut.Poll(&waker, out.Get()));
  EXPECT_EQ(std::move(out).AssumeInitAndTakeValue().field, 123);
}

TEST(AsyncFnTest, DoNothing) {
  NoopWaker waker;
  crubit::DynErasedFuture<void> fut = async_fn::do_nothing();
  EXPECT_TRUE(fut.Poll(&waker, nullptr));
}

TEST(AsyncFnTest, GetPending) {
  NoopWaker waker;
  crubit::DynErasedFuture<int32_t> fut = async_fn::pend_5_times();
  crubit::Slot<int32_t> out;
  for (int i = 0; i < 5; ++i) {
    ASSERT_FALSE(fut.Poll(&waker, out.Get()));
  }
  ASSERT_TRUE(fut.Poll(&waker, out.Get()));
  EXPECT_EQ(std::move(out).AssumeInitAndTakeValue(), 42);
}

TEST(AsyncFnTest, ReturnCppLayoutEquivalent) {
  NoopWaker waker;
  crubit::DynErasedFuture<crubit::test::AsyncFnCppLayoutEquivalent> fut =
      async_fn::return_cpp_layout_equivalent(456);
  crubit::Slot<crubit::test::AsyncFnCppLayoutEquivalent> out;
  ASSERT_TRUE(fut.Poll(&waker, out.Get()));
  EXPECT_EQ(std::move(out).AssumeInitAndTakeValue().get_x(), 456);
}

// The following functions are not generated in C++ due to lack of support,
// exactly as expected:
//
// 1. return_bridged_convertible:
//    Fails to generate bindings because Crubit correctly rejects async
//    functions returning bridged types that require conversion thunks.
//
// 2. return_unmovable:
//    Fails to generate bindings because `NotCppMovable` does not implement
//    Default, so it lacks a C++ move constructor. While this works for
//    non-async fn (via Slot), it is correctly rejected for async fn since the
//    future outputs by value.
//
// 3. return_impl_future:
//    Fails to generate bindings because `impl Future` is not yet supported.
//
// 4. return_box_dyn_future:
//    Fails to generate bindings because `Box` is not yet supported.
//
// 5. non_send_return:
//    Fails to generate bindings because Crubit explicitly enforces that async
//    functions must return a Send future.

}  // namespace
