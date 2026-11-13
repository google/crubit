// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/never.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace rs {
namespace {

static_assert(!std::is_default_constructible_v<never_t>);
static_assert(std::is_copy_constructible_v<never_t>);
static_assert(std::is_move_constructible_v<never_t>);
static_assert(std::is_trivially_copy_constructible_v<never_t>);
static_assert(std::is_trivially_move_constructible_v<never_t>);
static_assert(std::is_copy_assignable_v<never_t>);
static_assert(std::is_move_assignable_v<never_t>);
static_assert(std::is_trivially_copy_assignable_v<never_t>);
static_assert(std::is_trivially_move_assignable_v<never_t>);
static_assert(std::is_trivially_destructible_v<never_t>);
static_assert(std::is_trivially_copyable_v<never_t>);
static_assert(is_never_t_v<never_t>);
static_assert(!is_never_t_v<int>);
static_assert(is_never_t_v<const never_t&>);

}  // namespace
}  // namespace rs
