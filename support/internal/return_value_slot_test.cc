// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/return_value_slot.h"

#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/log/check.h"

namespace crubit {
namespace {

constexpr int kUninitializedState = -1;
constexpr int kMovedAwayState = -2;
constexpr int kDestroyedState = -3;
struct MonitoringHelper {
  MonitoringHelper() = delete;
  MonitoringHelper(const MonitoringHelper&) = delete;
  MonitoringHelper& operator=(const MonitoringHelper&) = delete;

  explicit MonitoringHelper(int new_state, std::vector<int>* destroyed_states)
      : state(new_state), destroyed_states(destroyed_states) {
    CHECK_NE(state, kMovedAwayState);
    CHECK_NE(state, kDestroyedState);
  }

  MonitoringHelper(MonitoringHelper&& other) {
    state = other.state;
    destroyed_states = other.destroyed_states;
    other.state = kMovedAwayState;
  }

  MonitoringHelper& operator=(MonitoringHelper&& other) {
    // `operator=` runs on an initialized object, and receives an initialized
    // object (but either may be moved-from).
    CHECK_NE(state, kUninitializedState);
    CHECK_NE(state, kDestroyedState);
    CHECK_NE(other.state, kUninitializedState);
    CHECK_NE(other.state, kDestroyedState);

    // Pretend to destroy old field values.
    destroyed_states->push_back(state);

    state = other.state;
    destroyed_states = other.destroyed_states;
    other.state = kMovedAwayState;
    return *this;
  }

  ~MonitoringHelper() {
    // The destructor runs on an initialized object (but it may be moved-from).
    CHECK_NE(state, kUninitializedState);
    CHECK_NE(state, kDestroyedState);
    destroyed_states->push_back(state);
    state = kDestroyedState;
  }

  int state;
  std::vector<int>* destroyed_states;
};

TEST(ReturnValueSlot, Test) {
  std::vector<int> destroyed_states;

  constexpr int kInitialValue = 1;
  constexpr int kReturnedValue = 2;

  MonitoringHelper return_value(kInitialValue, &destroyed_states);

  {
    // At this point `slot` is in an uninitialized state.
    ReturnValueSlot<MonitoringHelper> slot;
    MonitoringHelper* slot_ptr = slot.Get();
    slot_ptr->state = kUninitializedState;
    slot_ptr->destroyed_states = &destroyed_states;
    // No destructors should run up to this point.
    EXPECT_THAT(destroyed_states, testing::IsEmpty());

    // Initialize the memory.
    new (slot_ptr) MonitoringHelper(kReturnedValue, &destroyed_states);
    EXPECT_THAT(destroyed_states, testing::IsEmpty());

    // Move the return value from `slot` to `return_value`.
    return_value = std::move(slot).AssumeInitAndTakeValue();
    // Move assignment will destroy fields of the original lhs value that gets
    // overwritten by the assignment - this is where `kInitialValue` comes from.
    //
    // AssumeInitAndTakeValue will destroy `ReturnValueSlot::value_` in a
    // kMovedAwayState.
    //
    // Additionally, a temporary `MonitoringHelper` value in a moved-away state
    // will be destroyed.
    EXPECT_THAT(
        destroyed_states,
        testing::ElementsAre(kMovedAwayState, kInitialValue, kMovedAwayState));
    // The value inside `ReturnValueSlot` (pointed to by `slot_ptr`) should be
    // in a `kDestroyedState` state at this point due to
    // `AssumeInitAndTakeValue()`.
    EXPECT_EQ(kDestroyedState, slot_ptr->state);
    EXPECT_EQ(kReturnedValue, return_value.state);
  }

  EXPECT_EQ(kReturnedValue, return_value.state);
}

}  // namespace
}  // namespace crubit
