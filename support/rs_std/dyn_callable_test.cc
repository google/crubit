// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/dyn_callable.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "absl/functional/any_invocable.h"

namespace {

static_assert(std::is_default_constructible_v<rs_std::DynCallable<void()>>);
static_assert(!std::is_copy_constructible_v<rs_std::DynCallable<void()>>);
static_assert(!std::is_copy_assignable_v<rs_std::DynCallable<void()>>);
static_assert(std::is_move_constructible_v<rs_std::DynCallable<void()>>);
static_assert(std::is_move_assignable_v<rs_std::DynCallable<void()>>);
static_assert(std::is_destructible_v<rs_std::DynCallable<void()>>);

using Fn = rs_std::DynCallable<void() const>;
static_assert(std::is_invocable_v<Fn&>);
static_assert(std::is_invocable_v<const Fn&>);
static_assert(std::is_invocable_v<Fn&&>);
static_assert(std::is_invocable_v<const Fn&&>);

using FnMut = rs_std::DynCallable<void()>;
static_assert(std::is_invocable_v<FnMut&>);
static_assert(!std::is_invocable_v<const FnMut&>);
static_assert(std::is_invocable_v<FnMut&&>);
static_assert(!std::is_invocable_v<const FnMut&&>);

using FnOnce = rs_std::DynCallable<void() &&>;
static_assert(!std::is_invocable_v<FnOnce&>);
static_assert(!std::is_invocable_v<const FnOnce&>);
static_assert(std::is_invocable_v<FnOnce&&>);
static_assert(!std::is_invocable_v<const FnOnce&&>);

TEST(DynCallableTest, CheckEmpty) {
  rs_std::DynCallable<void()> empty;
  EXPECT_EQ(empty, nullptr);
  EXPECT_EQ(nullptr, empty);
  EXPECT_FALSE(empty);
}

TEST(DynCallableTest, Move) {
  rs_std::DynCallable<void()> empty;
  rs_std::DynCallable<void()> empty_copy(std::move(empty));
  EXPECT_FALSE(empty_copy);
}

TEST(DynCallableTest, CheckThatInternalDetailsAreReused) {
  rs_std::DynCallable<void()> dyn_callable;
  // When AnyInvocable is created from a callable using the template
  // constructor, it is considered non-empty. However, we should witness that
  // creating it from an empty DynCallable results in an empty AnyInvocable,
  // because we use the special operator AnyInvocable (which reuses internal
  // details) instead of the template constructor.
  absl::AnyInvocable<void()> any_invocable = std::move(dyn_callable);
  EXPECT_FALSE(any_invocable);
}

}  // namespace
