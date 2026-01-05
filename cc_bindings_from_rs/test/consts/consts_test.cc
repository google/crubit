// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/consts/consts.h"

#include "gtest/gtest.h"

namespace {

TEST(ConstsTest, AllAreExpected) {
  static_assert(consts::kRustTrue);
  static_assert(!consts::kRustFalse);
  static_assert(consts::kRustInt8Min == INT8_MIN);
  static_assert(consts::kRustInt8Max == INT8_MAX);
  static_assert(consts::kRustInt16Min == INT16_MIN);
  static_assert(consts::kRustInt16Max == INT16_MAX);
  static_assert(consts::kRustInt32Min == INT32_MIN);
  static_assert(consts::kRustInt32Max == INT32_MAX);
  static_assert(consts::kRustInt64Min == INT64_MIN);
  static_assert(consts::kRustInt64Max == INT64_MAX);
  static_assert(consts::kRustUint8Min == 0);
  static_assert(consts::kRustUint8Max == UINT8_MAX);
  static_assert(consts::kRustUint16Min == 0);
  static_assert(consts::kRustUint16Max == UINT16_MAX);
  static_assert(consts::kRustUint32Min == 0);
  static_assert(consts::kRustUint32Max == UINT32_MAX);
  static_assert(consts::kRustUint64Min == 0);
  static_assert(consts::kRustUint64Max == UINT64_MAX);
  // NOTE: FLT_MIN and DBL_MIN in C++ are not the furthest negative numbers,
  // they're the smallest positive values. There are no standard constants for
  // the furthest negative floating-point values, so no assertion is made here.
  static_assert(consts::kRustF32Max == FLT_MAX);
  static_assert(consts::kRustF64Max == DBL_MAX);

  static_assert(consts::TyWithAssocConsts::kAssoc42 == 42);
}

}  // namespace
