// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/consts/consts.h"

#include <bit>
#include <cstdint>

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
  static_assert(consts::RUST_F32_MIN_POSITIVE == FLT_MIN);
  static_assert(consts::RUST_F32_EPSILON == FLT_EPSILON);
  static_assert(consts::RUST_F64_MAX == DBL_MAX);
  static_assert(consts::RUST_F64_MIN_POSITIVE == DBL_MIN);
  static_assert(consts::RUST_F64_EPSILON == DBL_EPSILON);
  static_assert(consts::FLOAT_32 == 0.125f);
  static_assert(consts::FLOAT_32_WHOLE == 64.0f);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_NEG_ZERO) ==
                0x80000000u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_MIN_SUBNORMAL) == 1u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_MAX_SUBNORMAL) ==
                0x007fffffu);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_NEG_SUBNORMAL) ==
                0x80000001u);
  static_assert(consts::FLOAT_32_INFINITY == __builtin_inff());
  static_assert(consts::FLOAT_32_NEG_INFINITY == -__builtin_inff());
  static_assert(consts::FLOAT_32_NAN != consts::FLOAT_32_NAN);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_NAN) == 0x7fc00000u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_NEG_NAN) ==
                0xffc00000u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_PAYLOAD_NAN) ==
                0x7fc00042u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_NEG_PAYLOAD_NAN) ==
                0xffc00042u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_MAX_PAYLOAD_NAN) ==
                0x7fffffffu);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_SIGNALING_NAN) ==
                0x7f800001u);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_32_NEG_SIGNALING_NAN) ==
                0xff800001u);
  static_assert(consts::FLOAT_64 == 0.0078125);
  static_assert(consts::FLOAT_64_WHOLE == 64.0);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_NEG_ZERO) ==
                0x8000000000000000ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_MIN_SUBNORMAL) ==
                1ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_MAX_SUBNORMAL) ==
                0x000fffffffffffffull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_NEG_SUBNORMAL) ==
                0x8000000000000001ull);
  static_assert(consts::FLOAT_64_INFINITY == __builtin_inf());
  static_assert(consts::FLOAT_64_NEG_INFINITY == -__builtin_inf());
  static_assert(consts::FLOAT_64_NAN != consts::FLOAT_64_NAN);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_NAN) ==
                0x7ff8000000000000ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_NEG_NAN) ==
                0xfff8000000000000ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_PAYLOAD_NAN) ==
                0x7ff8000000000042ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_NEG_PAYLOAD_NAN) ==
                0xfff8000000000042ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_MAX_PAYLOAD_NAN) ==
                0x7fffffffffffffffull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_SIGNALING_NAN) ==
                0x7ff0000000000001ull);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_NEG_SIGNALING_NAN) ==
                0xfff0000000000001ull);

  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_STRUCT_CONST.x) ==
                0x80000000u);
  static_assert(consts::FLOAT_STRUCT_CONST.y == __builtin_inff());
  static_assert(consts::FLOAT_ARRAY_CONST[0] == 64.0f);
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_ARRAY_CONST[1]) ==
                0x80000000u);
  static_assert(consts::FLOAT_ARRAY_CONST[2] == -__builtin_inff());
  static_assert(std::bit_cast<uint32_t>(consts::FLOAT_ARRAY_CONST[3]) ==
                0x7fc00000u);
  static_assert(std::bit_cast<uint64_t>(consts::FLOAT_64_ARRAY_CONST[0]) ==
                0x8000000000000000ull);
  static_assert(consts::FLOAT_64_ARRAY_CONST[1] == __builtin_inf());

  static_assert(consts::TyWithAssocConsts::ASSOC_42 == 42);
  static_assert(consts::TyWithAssocConsts::ASSOC_POINT.x == 5);
  static_assert(consts::TyWithAssocConsts::ASSOC_POINT.y == 6);

  static_assert(consts::POINT_CONST.x == 10);
  static_assert(consts::POINT_CONST.y == -20);

  static_assert(consts::Point::ZERO.x == 0);
  static_assert(consts::Point::ZERO.y == 0);

  static_assert(consts::TUPLE_STRUCT_CONST.__field0 == 123);
  static_assert(consts::TUPLE_STRUCT_CONST.__field1 == 456);

  static_assert(consts::NESTED_STRUCT_CONST.point.x == 1);
  static_assert(consts::NESTED_STRUCT_CONST.point.y == 2);
  static_assert(consts::NESTED_STRUCT_CONST.tuple.__field0 == 3);
  static_assert(consts::NESTED_STRUCT_CONST.tuple.__field1 == 4);

  static_assert(consts::ARRAY_CONST[0] == 10);
  static_assert(consts::ARRAY_CONST[1] == 20);
  static_assert(consts::ARRAY_CONST[2] == 30);

  static_assert(consts::STRUCT_WITH_ARRAY_CONST.values[0] == 100);
  static_assert(consts::STRUCT_WITH_ARRAY_CONST.values[1] == 200);

  static_assert(std::is_default_constructible_v<consts::StructWithStr>);
  consts::StructWithStr default_str{};
  EXPECT_EQ(default_str.msg, "");
  EXPECT_EQ(default_str.count, 0);

  EXPECT_EQ(consts::STRUCT_WITH_STR_CONST.msg, "hello world");
  EXPECT_EQ(consts::STRUCT_WITH_STR_CONST.count, 42);

  static_assert(!std::is_default_constructible_v<consts::StructWithRef>);
}

TEST(ConstsTest, FloatRoundtrip) {
  auto check_f32 = [](float val, uint32_t expected_bits) {
    EXPECT_EQ(std::bit_cast<uint32_t>(val), expected_bits);
    EXPECT_EQ(consts::f32_to_bits(val), expected_bits);
    EXPECT_EQ(std::bit_cast<uint32_t>(consts::roundtrip_f32(val)),
              expected_bits);
  };
  check_f32(consts::RUST_F32_MIN, 0xff7fffffu);
  check_f32(consts::RUST_F32_MAX, 0x7f7fffffu);
  check_f32(consts::RUST_F32_MIN_POSITIVE, 0x00800000u);
  check_f32(consts::RUST_F32_EPSILON, 0x34000000u);
  check_f32(consts::FLOAT_32, 0x3e000000u);
  check_f32(consts::FLOAT_32_WHOLE, 0x42800000u);
  check_f32(consts::FLOAT_32_NEG_ZERO, 0x80000000u);
  check_f32(consts::FLOAT_32_MIN_SUBNORMAL, 0x00000001u);
  check_f32(consts::FLOAT_32_MAX_SUBNORMAL, 0x007fffffu);
  check_f32(consts::FLOAT_32_NEG_SUBNORMAL, 0x80000001u);
  check_f32(consts::FLOAT_32_INFINITY, 0x7f800000u);
  check_f32(consts::FLOAT_32_NEG_INFINITY, 0xff800000u);
  check_f32(consts::FLOAT_32_NAN, 0x7fc00000u);
  check_f32(consts::FLOAT_32_NEG_NAN, 0xffc00000u);
  check_f32(consts::FLOAT_32_PAYLOAD_NAN, 0x7fc00042u);
  check_f32(consts::FLOAT_32_NEG_PAYLOAD_NAN, 0xffc00042u);
  check_f32(consts::FLOAT_32_MAX_PAYLOAD_NAN, 0x7fffffffu);
  check_f32(consts::FLOAT_32_SIGNALING_NAN, 0x7f800001u);
  check_f32(consts::FLOAT_32_NEG_SIGNALING_NAN, 0xff800001u);

  auto check_f64 = [](double val, uint64_t expected_bits) {
    EXPECT_EQ(std::bit_cast<uint64_t>(val), expected_bits);
    EXPECT_EQ(consts::f64_to_bits(val), expected_bits);
    EXPECT_EQ(std::bit_cast<uint64_t>(consts::roundtrip_f64(val)),
              expected_bits);
  };
  check_f64(consts::RUST_F64_MIN, 0xffefffffffffffffull);
  check_f64(consts::RUST_F64_MAX, 0x7fefffffffffffffull);
  check_f64(consts::RUST_F64_MIN_POSITIVE, 0x0010000000000000ull);
  check_f64(consts::RUST_F64_EPSILON, 0x3cb0000000000000ull);
  check_f64(consts::FLOAT_64, 0x3f80000000000000ull);
  check_f64(consts::FLOAT_64_WHOLE, 0x4050000000000000ull);
  check_f64(consts::FLOAT_64_NEG_ZERO, 0x8000000000000000ull);
  check_f64(consts::FLOAT_64_MIN_SUBNORMAL, 0x0000000000000001ull);
  check_f64(consts::FLOAT_64_MAX_SUBNORMAL, 0x000fffffffffffffull);
  check_f64(consts::FLOAT_64_NEG_SUBNORMAL, 0x8000000000000001ull);
  check_f64(consts::FLOAT_64_INFINITY, 0x7ff0000000000000ull);
  check_f64(consts::FLOAT_64_NEG_INFINITY, 0xfff0000000000000ull);
  check_f64(consts::FLOAT_64_NAN, 0x7ff8000000000000ull);
  check_f64(consts::FLOAT_64_NEG_NAN, 0xfff8000000000000ull);
  check_f64(consts::FLOAT_64_PAYLOAD_NAN, 0x7ff8000000000042ull);
  check_f64(consts::FLOAT_64_NEG_PAYLOAD_NAN, 0xfff8000000000042ull);
  check_f64(consts::FLOAT_64_MAX_PAYLOAD_NAN, 0x7fffffffffffffffull);
  check_f64(consts::FLOAT_64_SIGNALING_NAN, 0x7ff0000000000001ull);
  check_f64(consts::FLOAT_64_NEG_SIGNALING_NAN, 0xfff0000000000001ull);
}

}  // namespace
