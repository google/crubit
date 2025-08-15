// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/consts/consts.h"

#include "gtest/gtest.h"

namespace {

TEST(ConstsTest, AllAreExpected) {
  static_assert(consts::RUST_TRUE);
  static_assert(!consts::RUST_FALSE);
  static_assert(consts::RUST_INT8_MIN == INT8_MIN);
  static_assert(consts::RUST_INT8_MAX == INT8_MAX);
  static_assert(consts::RUST_INT16_MIN == INT16_MIN);
  static_assert(consts::RUST_INT16_MAX == INT16_MAX);
  static_assert(consts::RUST_INT32_MIN == INT32_MIN);
  static_assert(consts::RUST_INT32_MAX == INT32_MAX);
  static_assert(consts::RUST_INT64_MIN == INT64_MIN);
  static_assert(consts::RUST_INT64_MAX == INT64_MAX);
  static_assert(consts::RUST_UINT8_MIN == 0);
  static_assert(consts::RUST_UINT8_MAX == UINT8_MAX);
  static_assert(consts::RUST_UINT16_MIN == 0);
  static_assert(consts::RUST_UINT16_MAX == UINT16_MAX);
  static_assert(consts::RUST_UINT32_MIN == 0);
  static_assert(consts::RUST_UINT32_MAX == UINT32_MAX);
  static_assert(consts::RUST_UINT64_MIN == 0);
  static_assert(consts::RUST_UINT64_MAX == UINT64_MAX);
  // NOTE: FLT_MIN and DBL_MIN in C++ are not the furthest negative numbers,
  // they're the smallest positive values. There are no standard constants for
  // the furthest negative floating-point values, so no assertion is made here.
  static_assert(consts::RUST_F32_MAX == FLT_MAX);
  static_assert(consts::RUST_F64_MAX == DBL_MAX);

  static_assert(consts::TyWithAssocConsts::ASSOC_42 == 42);
}

}  // namespace
