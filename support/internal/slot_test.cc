// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/slot.h"

#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/base/attributes.h"
#include "absl/log/check.h"

namespace crubit {
namespace {

using ::testing::_;
using ::testing::ElementsAre;
using ::testing::IsEmpty;

constexpr int kUninitializedState = -1;
constexpr int kMoveConstructedFromState = -2;
constexpr int kMoveAssignedFromState = -3;
constexpr int kRelocatedState = -4;
constexpr int kDestroyedState = -5;

struct ABSL_ATTRIBUTE_TRIVIAL_ABI MonitoringHelper {
  MonitoringHelper() = delete;
  MonitoringHelper(const MonitoringHelper&) = delete;
  MonitoringHelper& operator=(const MonitoringHelper&) = delete;

  explicit MonitoringHelper(int new_state, std::vector<int>* destroyed_states,
                            std::vector<void*>* destroyed_locations)
      : state(new_state),
        destroyed_states(destroyed_states),
        destroyed_locations(destroyed_locations) {
    CHECK_NE(state, kMoveConstructedFromState);
    CHECK_NE(state, kMoveAssignedFromState);
    CHECK_NE(state, kRelocatedState);
    CHECK_NE(state, kDestroyedState);
  }

  MonitoringHelper(MonitoringHelper&& other) {
    state = other.state;
    destroyed_states = other.destroyed_states;
    destroyed_locations = other.destroyed_locations;
    other.state = kMoveConstructedFromState;
  }

  MonitoringHelper& operator=(MonitoringHelper&& other) {
    // `operator=` runs on an initialized object, and receives an initialized
    // object (but either may be moved-from).
    CHECK_NE(state, kUninitializedState);
    CHECK_NE(state, kRelocatedState);
    CHECK_NE(state, kDestroyedState);
    CHECK_NE(other.state, kUninitializedState);
    CHECK_NE(other.state, kRelocatedState);
    CHECK_NE(other.state, kDestroyedState);

    // Pretend to destroy old field values.
    destroyed_states->push_back(state);
    destroyed_locations->push_back(this);

    state = other.state;
    destroyed_states = other.destroyed_states;
    destroyed_locations = other.destroyed_locations;
    other.state = kMoveAssignedFromState;
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
  std::vector<void*>* destroyed_locations;
};

// The same as MonitoringHelper, but with an UnsafeRelocateTag constructor.
struct ABSL_ATTRIBUTE_TRIVIAL_ABI RelocatableMonitoringHelper
    : public MonitoringHelper {
  explicit RelocatableMonitoringHelper(int new_state,
                                       std::vector<int>* destroyed_states,
                                       std::vector<void*>* destroyed_locations)
      : MonitoringHelper(new_state, destroyed_states, destroyed_locations) {}

  RelocatableMonitoringHelper(crubit::UnsafeRelocateTag,
                              RelocatableMonitoringHelper&& other)
      : MonitoringHelper(std::move(other)) {
    other.state = kRelocatedState;
  }
};

template <typename T>
class SlotTest : public testing::Test {};

using MyTypes = ::testing::Types<MonitoringHelper, RelocatableMonitoringHelper>;
TYPED_TEST_SUITE(SlotTest, MyTypes);

TYPED_TEST(SlotTest, Test) {
  std::vector<int> destroyed_states;
  std::vector<void*> destroyed_locations;

  constexpr int kInitialValue = 1;
  constexpr int kReturnedValue = 2;

  TypeParam return_value(kInitialValue, &destroyed_states,
                         &destroyed_locations);

  {
    // At this point `slot` is in an uninitialized state.
    Slot<TypeParam> slot;
    TypeParam* slot_ptr = slot.Get();
    slot_ptr->state = kUninitializedState;
    slot_ptr->destroyed_states = &destroyed_states;
    slot_ptr->destroyed_locations = &destroyed_locations;
    // No destructors should run up to this point.
    EXPECT_THAT(destroyed_states, IsEmpty());
    EXPECT_THAT(destroyed_locations, IsEmpty());

    // Initialize the memory.
    new (slot_ptr)
        TypeParam(kReturnedValue, &destroyed_states, &destroyed_locations);
    EXPECT_THAT(destroyed_states, IsEmpty());
    EXPECT_THAT(destroyed_locations, IsEmpty());

    // Move the return value from `slot` to `return_value`.
    return_value = std::move(slot).AssumeInitAndTakeValue();
    if constexpr (std::is_same_v<TypeParam, MonitoringHelper>) {
      // Move assignment will destroy fields of the original lhs value that gets
      // overwritten by the assignment - this is where `kInitialValue` comes
      // from.
      //
      // AssumeInitAndTakeValue will destroy `Slot::value_` in a
      // kMovedAwayState.  This is asserted by checking that `slot_ptr` is
      // covered by `destroyed_locations`.
      //
      // Additionally, the temporary return value of AssumeInitAndTakeValue
      // will be destroyed (in a kMoveAssignedFromState).
      EXPECT_THAT(destroyed_states,
                  ElementsAre(kMoveConstructedFromState, kInitialValue,
                              kMoveAssignedFromState));
      EXPECT_THAT(destroyed_locations, ElementsAre(slot_ptr, &return_value, _));
    } else {
      // When there is an UnsafeRelocateTag constructor, we don't need the moved
      // value inside of AssumeInitAndTakeValue. So the only destroyed values
      // are the assignment target and the temporary.
      EXPECT_THAT(destroyed_states, ElementsAre(kInitialValue, _));
      EXPECT_THAT(destroyed_locations, ElementsAre(&return_value, _));
    }

    EXPECT_EQ(kReturnedValue, return_value.state);
  }

  EXPECT_EQ(kReturnedValue, return_value.state);
}

}  // namespace
}  // namespace crubit
