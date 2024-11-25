// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/slot.h"

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

  explicit MonitoringHelper(int new_state, std::vector<int>* destroyed_states,
                            std::vector<MonitoringHelper*>* destroyed_locations)
      : state(new_state),
        destroyed_states(destroyed_states),
        destroyed_locations(destroyed_locations) {
    CHECK_NE(state, kMovedAwayState);
    CHECK_NE(state, kDestroyedState);
  }

  MonitoringHelper(MonitoringHelper&& other) {
    state = other.state;
    destroyed_states = other.destroyed_states;
    destroyed_locations = other.destroyed_locations;
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
    destroyed_locations->push_back(this);

    state = other.state;
    destroyed_states = other.destroyed_states;
    destroyed_locations = other.destroyed_locations;
    other.state = kMovedAwayState;
    return *this;
  }

  ~MonitoringHelper() {
    // The destructor runs on an initialized object (but it may be moved-from).
    CHECK_NE(state, kUninitializedState);
    CHECK_NE(state, kDestroyedState);
    destroyed_states->push_back(state);
    destroyed_locations->push_back(this);
    state = kDestroyedState;
  }

  int state;
  std::vector<int>* destroyed_states;
  std::vector<MonitoringHelper*>* destroyed_locations;
};

TEST(Slot, Test) {
  std::vector<int> destroyed_states;
  std::vector<MonitoringHelper*> destroyed_locations;

  constexpr int kInitialValue = 1;
  constexpr int kReturnedValue = 2;

  MonitoringHelper return_value(kInitialValue, &destroyed_states,
                                &destroyed_locations);

  {
    // At this point `slot` is in an uninitialized state.
    Slot<MonitoringHelper> slot;
    MonitoringHelper* slot_ptr = slot.Get();
    slot_ptr->state = kUninitializedState;
    slot_ptr->destroyed_states = &destroyed_states;
    slot_ptr->destroyed_locations = &destroyed_locations;
    // No destructors should run up to this point.
    EXPECT_THAT(destroyed_states, testing::IsEmpty());
    EXPECT_THAT(destroyed_locations, testing::IsEmpty());

    // Initialize the memory.
    new (slot_ptr) MonitoringHelper(kReturnedValue, &destroyed_states,
                                    &destroyed_locations);
    EXPECT_THAT(destroyed_states, testing::IsEmpty());
    EXPECT_THAT(destroyed_locations, testing::IsEmpty());

    // Move the return value from `slot` to `return_value`.
    return_value = std::move(slot).AssumeInitAndTakeValue();
    // Move assignment will destroy fields of the original lhs value that gets
    // overwritten by the assignment - this is where `kInitialValue` comes from.
    //
    // AssumeInitAndTakeValue will destroy `Slot::value_` in a
    // kMovedAwayState.  This is asserted by checking that `slot_ptr` is covered
    // by `destroyed_locations`.
    //
    // Additionally, a temporary `MonitoringHelper` value in a moved-away state
    // will be destroyed.
    EXPECT_THAT(
        destroyed_states,
        testing::ElementsAre(kMovedAwayState, kInitialValue, kMovedAwayState));
    EXPECT_THAT(destroyed_locations,
                testing::ElementsAre(slot_ptr, testing::_, testing::_));
    EXPECT_EQ(kReturnedValue, return_value.state);
  }

  EXPECT_EQ(kReturnedValue, return_value.state);
}

}  // namespace
}  // namespace crubit
