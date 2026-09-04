// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/callables/callables.h"

#include <cstdint>
#include <memory>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace {

using ::testing::Eq;

int FreeAddOne(int x) { return x + 1; }

TEST(CallablesTest, CallDynFn) {
  auto result = callables::call_dyn_fn([](int x) { return x * 2; }, 21);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallDynFnMut) {
  int total = 0;
  auto lambda = [&total](int x) mutable {
    total += x;
    return total;
  };
  EXPECT_THAT(callables::call_dyn_fn_mut(lambda, 10), Eq(10));
  EXPECT_THAT(callables::call_dyn_fn_mut(lambda, 32), Eq(42));
  EXPECT_THAT(total, Eq(42));
}

TEST(CallablesTest, CallBoxDynFn) {
  auto result = callables::call_box_dyn_fn([](int x) { return x + 10; }, 32);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallBoxDynFnMut) {
  int offset = 10;
  auto result = callables::call_box_dyn_fn_mut(
      [offset](int x) mutable { return x + offset; }, 32);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallBoxDynFnOnce) {
  auto ptr = std::make_unique<int>(10);
  auto result = callables::call_box_dyn_fn_once(
      [p = std::move(ptr)](int x) { return x + *p; }, 32);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallImplFn) {
  auto result = callables::call_impl_fn([](int x) { return x * 3; }, 14);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallImplFnWithFunctionPointer) {
  auto result = callables::call_impl_fn(FreeAddOne, 41);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallImplFnMut) {
  int count = 0;
  auto lambda = [&count](int x) mutable {
    count += x;
    return count;
  };
  EXPECT_THAT(callables::call_impl_fn_mut(lambda, 20), Eq(20));
  EXPECT_THAT(callables::call_impl_fn_mut(lambda, 22), Eq(42));
}

TEST(CallablesTest, CallImplFnOnce) {
  auto result = callables::call_impl_fn_once([](int x) { return x + 1; }, 41);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallImplFnOnceStatic) {
  auto ptr = std::make_unique<int>(22);
  auto result = callables::call_impl_fn_once_static(
      [p = std::move(ptr)](int x) { return x + *p; }, 20);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallTwoArgs) {
  auto result =
      callables::call_two_args([](int a, int b) { return a * b; }, 6, 7);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallVoid) {
  int value = 0;
  callables::call_void([&value](int x) { value = x; }, 42);
  EXPECT_THAT(value, Eq(42));
}

TEST(CallablesTest, CallVoidMut) {
  int value = 0;
  auto lambda = [&value](int x) mutable { value += x; };
  callables::call_void_mut(lambda, 10);
  callables::call_void_mut(lambda, 32);
  EXPECT_THAT(value, Eq(42));
}

TEST(CallablesTest, CallVoidOnce) {
  int value = 0;
  callables::call_void_once([&value](int x) { value = x; }, 42);
  EXPECT_THAT(value, Eq(42));
}

TEST(CallablesTest, DestructorRunsOnDrop) {
  auto tracker = std::make_shared<int>(99);
  EXPECT_THAT(tracker.use_count(), Eq(1));
  {
    auto result = callables::call_impl_fn_once_static(
        [tracker](int x) { return *tracker + x; }, 1);
    EXPECT_THAT(result, Eq(100));
  }
  // After the call completes and the Rust Box is dropped, the C++ closure was
  // destroyed.
  EXPECT_THAT(tracker.use_count(), Eq(1));
}

TEST(CallablesTest, CallWithPoint) {
  callables::Point pt{.x = 10, .y = 32};
  auto result = callables::call_with_point(
      [](callables::Point p) {
        return callables::Point{.x = p.x * 2, .y = p.y * 2};
      },
      pt);
  EXPECT_THAT(result.x, Eq(20));
  EXPECT_THAT(result.y, Eq(64));
}

TEST(CallablesTest, CallPointToInt) {
  callables::Point pt{.x = 10, .y = 32};
  auto result = callables::call_point_to_int(
      [](callables::Point p) { return p.x + p.y; }, pt);
  EXPECT_THAT(result, Eq(42));
}

TEST(CallablesTest, CallIntToPoint) {
  auto result = callables::call_int_to_point(
      [](int x) { return callables::Point{.x = x, .y = x * 2}; }, 21);
  EXPECT_THAT(result.x, Eq(21));
  EXPECT_THAT(result.y, Eq(42));
}

TEST(CallablesTest, CallPointMut) {
  callables::Point pt{.x = 5, .y = 10};
  int invocation_count = 0;
  auto lambda = [&invocation_count](callables::Point p) mutable {
    ++invocation_count;
    return callables::Point{.x = p.x + invocation_count,
                            .y = p.y + invocation_count};
  };
  auto res1 = callables::call_point_mut(lambda, pt);
  EXPECT_THAT(res1.x, Eq(6));
  EXPECT_THAT(res1.y, Eq(11));
  auto res2 = callables::call_point_mut(lambda, pt);
  EXPECT_THAT(res2.x, Eq(7));
  EXPECT_THAT(res2.y, Eq(12));
  EXPECT_THAT(invocation_count, Eq(2));
}

TEST(CallablesTest, CallPointOnceStatic) {
  auto ptr = std::make_unique<int>(15);
  callables::Point pt{.x = 10, .y = 20};
  auto result = callables::call_point_once_static(
      [p = std::move(ptr)](callables::Point pt) {
        return callables::Point{.x = pt.x + *p, .y = pt.y + *p};
      },
      pt);
  EXPECT_THAT(result.x, Eq(25));
  EXPECT_THAT(result.y, Eq(35));
}

TEST(CallablesTest, CallTwoPoints) {
  callables::Point p1{.x = 1, .y = 2};
  callables::Point p2{.x = 3, .y = 4};
  auto result = callables::call_two_points(
      [](callables::Point a, callables::Point b) {
        return callables::Point{.x = a.x + b.x, .y = a.y + b.y};
      },
      p1, p2);
  EXPECT_THAT(result.x, Eq(4));
  EXPECT_THAT(result.y, Eq(6));
}

TEST(CallablesTest, CallPointVoid) {
  int sum = 0;
  callables::Point pt{.x = 20, .y = 22};
  callables::call_point_void([&sum](callables::Point p) { sum = p.x + p.y; },
                             pt);
  EXPECT_THAT(sum, Eq(42));
}

TEST(CallablesTest, CallWithStr) {
  auto result = callables::call_with_str(
      [](rs_std::StrRef s) -> int {
        return static_cast<int>(s.to_string_view().size());
      },
      rs_std::StrRef("hello world"));
  EXPECT_THAT(result, Eq(11));
}

TEST(CallablesTest, CallWithTupleOption) {
  auto result = callables::call_with_tuple_option(
      [](rs_std::Tuple<int, rs_std::Option<int>> t) {
        return rs_std::Tuple<int, rs_std::Option<int>>(std::make_tuple(
            t.__field0 * 2, rs_std::Option<int>(*t.__field1 * 3)));
      });
  EXPECT_THAT(result, Eq(42 * 2 + 100 * 3));
}

TEST(CallablesTest, CallImplWithTupleOption) {
  auto result = callables::call_impl_with_tuple_option(
      [](rs_std::Tuple<int, rs_std::Option<int>> t) {
        return rs_std::Tuple<int, rs_std::Option<int>>(std::make_tuple(
            t.__field0 + 5, rs_std::Option<int>(*t.__field1 + 5)));
      });
  EXPECT_THAT(result, Eq((10 + 5) + (20 + 5)));
}

TEST(CallablesTest, CallWithHrtbStr) {
  auto result = callables::call_with_hrtb_str(
      [](rs_std::StrRef s) -> int {
        return static_cast<int>(s.to_string_view().size());
      },
      rs_std::StrRef("higher-ranked"));
  EXPECT_THAT(result, Eq(13));
}

TEST(CallablesTest, CallWithHrtbStrToStr) {
  auto result = callables::call_with_hrtb_str_to_str(
      [](rs_std::StrRef s) -> rs_std::StrRef { return s; },
      rs_std::StrRef("echo"));
  EXPECT_THAT(result, Eq(4));
}

TEST(CallablesTest, CallWithStrToStr) {
  auto result = callables::call_with_str_to_str(
      [](rs_std::StrRef s) -> rs_std::StrRef { return s; },
      rs_std::StrRef("hello"));
  EXPECT_THAT(result.to_string_view(), Eq("hello"));
}

TEST(CallablesTest, CallbackHolderRetainsCapturesUntilDropped) {
  auto tracker = std::make_shared<int>(42);
  EXPECT_THAT(tracker.use_count(), Eq(1));

  auto holder = callables::CallbackHolder::new_();

  bool called = false;
  holder.set_callback([tracker, &called]() {
    called = true;
    EXPECT_THAT(*tracker, Eq(42));
  });

  // The lambda has been moved into the Box<dyn Fn()> inside the Rust struct.
  // The captured `tracker` is retained and kept alive by the Rust struct.
  EXPECT_THAT(tracker.use_count(), Eq(2));
  EXPECT_FALSE(called);

  holder.call();
  EXPECT_TRUE(called);

  // Still retained after calling.
  EXPECT_THAT(tracker.use_count(), Eq(2));

  // Drop the callback explicitly on the Rust struct.
  holder.drop_callback();

  // Now the Box<dyn Fn()> was dropped on the Rust side, which called the
  // destroyer and released the captured shared_ptr.
  EXPECT_THAT(tracker.use_count(), Eq(1));
}

TEST(CallablesTest, CallbackHolderDropsCapturesOnDestruction) {
  auto tracker = std::make_shared<int>(100);
  EXPECT_THAT(tracker.use_count(), Eq(1));

  {
    auto holder = callables::CallbackHolder::new_();
    holder.set_callback([tracker]() {});
    EXPECT_THAT(tracker.use_count(), Eq(2));
  }

  // After `holder` goes out of scope and is destroyed, the captured `tracker`
  // is released.
  EXPECT_THAT(tracker.use_count(), Eq(1));
}

TEST(CallablesTest, CallAndReturnNonMovable) {
  callables::NonCppMovable result = callables::call_and_return_non_movable(
      []() { return callables::NonCppMovable(42); });
  EXPECT_THAT(result.__field0, Eq(42));
}

TEST(CallablesTest, CallAndReturnNonMovableBoxed) {
  callables::NonCppMovable result =
      callables::call_and_return_non_movable_boxed(
          []() { return callables::NonCppMovable(123); });
  EXPECT_THAT(result.__field0, Eq(123));
}

TEST(CallablesTest, CallAndReturnNonMovableBoxFn) {
  callables::NonCppMovable result =
      callables::call_and_return_non_movable_box_fn(
          []() { return callables::NonCppMovable(456); });
  EXPECT_THAT(result.__field0, Eq(456));
}

TEST(CallablesTest, CallWithNonMovableRef) {
  callables::NonCppMovable x(999);
  int observed = 0;
  int ret = callables::call_with_non_movable_ref(
      [&](const callables::NonCppMovable* ref) { observed = ref->__field0; },
      x);
  EXPECT_THAT(observed, Eq(999));
  EXPECT_THAT(ret, Eq(999));
}

TEST(CallablesTest, CallWithMovableDrop) {
  int observed = 0;
  callables::call_with_movable_drop(
      [&](callables::CppMovableDrop x) { observed = x.__field0; }, 777);
  EXPECT_THAT(observed, Eq(777));
}

}  // namespace
